/* Linux-only test interposer. Never linked into the editor or used for production.
 * Eligible operations are confined to a marker-validated new harness directory,
 * this process's exact .mm3e-PID-N.tmp names and one whitelisted target filename.
 * Real operations use Linux syscalls directly; every injected return is explicit
 * in the JSONL log. This models error returns, not physical disk/power failure. */
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <stdatomic.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <sys/uio.h>
#include <unistd.h>

static char root_path[PATH_MAX], target_path[PATH_MAX], log_path[PATH_MAX];
static char arm_path[PATH_MAX], fault_mode[64], temp_prefix[64], linked_source[PATH_MAX];
static bool enabled;
static atomic_int fired, short_written, renamed, linked;
static atomic_ulong sequence;

static bool armed(void) {
    return enabled && syscall(SYS_faccessat, AT_FDCWD, arm_path, F_OK) == 0;
}

static void escaped(const char *input, char *output, size_t capacity) {
    size_t n = 0;
    if (!input) input = "";
    for (const unsigned char *p = (const unsigned char *)input; *p && n + 7 < capacity; ++p) {
        if (*p == '"' || *p == '\\') { output[n++] = '\\'; output[n++] = (char)*p; }
        else if (*p < 32) { n += (size_t)snprintf(output + n, capacity - n, "\\u%04x", *p); }
        else output[n++] = (char)*p;
    }
    output[n] = 0;
}

static void event(const char *stage, const char *call, const char *path, const char *other,
                  int fd, size_t requested, size_t real_requested, long result, int error,
                  bool injected, bool syscall_ran, bool after_success) {
    int saved_errno = errno;
    if (enabled) {
        char path_json[8192], other_json[8192], line[20000];
        escaped(path, path_json, sizeof path_json);
        escaped(other, other_json, sizeof other_json);
        unsigned long seq = atomic_fetch_add(&sequence, 1);
        int length = snprintf(line, sizeof line,
            "{\"seq\":%lu,\"pid\":%ld,\"mode\":\"%s\",\"stage\":\"%s\",\"call\":\"%s\","
            "\"path\":\"%s\",\"other\":\"%s\",\"fd\":%d,\"requested\":%zu,\"real_requested\":%zu,"
            "\"result\":%ld,\"errno\":%d,\"armed\":%s,\"injected\":%s,\"syscall_ran\":%s,\"after_actual_success\":%s}\n",
            seq, (long)getpid(), fault_mode, stage, call, path_json, other_json, fd, requested, real_requested,
            result, error, armed() ? "true" : "false", injected ? "true" : "false",
            syscall_ran ? "true" : "false", after_success ? "true" : "false");
        if (length > 0 && (size_t)length < sizeof line) {
            int logfd = (int)syscall(SYS_openat, AT_FDCWD, log_path, O_WRONLY | O_APPEND | O_CREAT | O_CLOEXEC | O_NOFOLLOW, 0600);
            if (logfd >= 0) {
                (void)syscall(SYS_write, logfd, line, (size_t)length);
                (void)syscall(SYS_close, logfd);
            }
        }
    }
    errno = saved_errno;
}

__attribute__((constructor)) static void configure(void) {
    const char *root = getenv("MM3E_STORAGE_FAULT_ROOT");
    const char *name = getenv("MM3E_STORAGE_FAULT_TARGET");
    const char *mode = getenv("MM3E_STORAGE_FAULT_MODE");
    const char *token = getenv("MM3E_STORAGE_FAULT_TOKEN");
    if (!root || !name || !mode || !token || root[0] != '/' || strlen(root) < 2 || strlen(root) > 1024) return;
    if (strcmp(name, "project.json") && strcmp(name, "no-clobber.png")) return;
    if (strlen(token) != 32 || strlen(mode) >= sizeof fault_mode) return;
    const char *modes[] = {"none", "write_enospc", "file_sync_eio", "dir_sync_eio", "rename_before", "rename_after", "cleanup_unlink"};
    bool supported = false;
    for (size_t i = 0; i < sizeof modes / sizeof modes[0]; ++i) if (!strcmp(mode, modes[i])) supported = true;
    if (!supported) return;
    char canonical[PATH_MAX], marker[PATH_MAX], content[64];
    struct stat st;
    if (!realpath(root, canonical) || strcmp(root, canonical) || stat(root, &st) || !S_ISDIR(st.st_mode) || st.st_uid != geteuid()) return;
    snprintf(marker, sizeof marker, "%s/.mm3e-fault-sandbox", root);
    int markerfd = (int)syscall(SYS_openat, AT_FDCWD, marker, O_RDONLY | O_CLOEXEC | O_NOFOLLOW | O_NONBLOCK);
    if (markerfd < 0) return;
    bool regular = fstat(markerfd, &st) == 0 && S_ISREG(st.st_mode);
    long count = regular ? syscall(SYS_read, markerfd, content, sizeof content) : -1;
    (void)syscall(SYS_close, markerfd);
    if (count != 32 || memcmp(content, token, 32)) return;
    snprintf(root_path, sizeof root_path, "%s", root);
    snprintf(target_path, sizeof target_path, "%s/%s", root, name);
    snprintf(log_path, sizeof log_path, "%s/fault-events.jsonl", root);
    snprintf(arm_path, sizeof arm_path, "%s/.fault-armed", root);
    snprintf(fault_mode, sizeof fault_mode, "%s", mode);
    snprintf(temp_prefix, sizeof temp_prefix, ".mm3e-%ld-", (long)getpid());
    enabled = true;
    event("loaded", "constructor", root_path, target_path, -1, 0, 0, 0, 0, false, false, false);
}

static bool own_temp(const char *path) {
    if (!enabled || !path) return false;
    size_t root_len = strlen(root_path);
    if (strncmp(path, root_path, root_len) || path[root_len] != '/') return false;
    const char *name = path + root_len + 1;
    size_t prefix = strlen(temp_prefix);
    if (strncmp(name, temp_prefix, prefix)) return false;
    name += prefix;
    const char *digits = name;
    while (*name >= '0' && *name <= '9') ++name;
    return name != digits && !strcmp(name, ".tmp");
}

static bool fd_path(int fd, char *path) {
    char proc[64];
    snprintf(proc, sizeof proc, "/proc/self/fd/%d", fd);
    ssize_t n = readlink(proc, path, PATH_MAX - 1);
    if (n < 0) return false;
    path[n] = 0;
    return true;
}

static bool take_fault(void) {
    int expected = 0;
    return atomic_compare_exchange_strong(&fired, &expected, 1);
}

ssize_t write(int fd, const void *buffer, size_t count) {
    char path[PATH_MAX];
    bool scoped = enabled && fd_path(fd, path) && own_temp(path);
    if (scoped && armed() && !strcmp(fault_mode, "write_enospc") && !atomic_load(&fired)) {
        if (count > 1 && atomic_exchange(&short_written, 1) == 0) {
            size_t prefix = count > 37 ? 37 : count / 2;
            long rc = syscall(SYS_write, fd, buffer, prefix);
            int error = rc < 0 ? errno : 0;
            event("staged_write", "write_short_prefix", path, NULL, fd, count, prefix, rc, error, false, true, false);
            errno = error;
            return rc;
        }
        if (take_fault()) {
            event("staged_write", "write", path, NULL, fd, count, 0, -1, ENOSPC, true, false, false);
            errno = ENOSPC;
            return -1;
        }
    }
    long rc = syscall(SYS_write, fd, buffer, count);
    int error = rc < 0 ? errno : 0;
    if (scoped) event("staged_write", "write", path, NULL, fd, count, count, rc, error, false, true, false);
    errno = error;
    return rc;
}

ssize_t writev(int fd, const struct iovec *iov, int count) {
    char path[PATH_MAX];
    bool scoped = enabled && fd_path(fd, path) && own_temp(path);
    if (scoped && armed() && !strcmp(fault_mode, "write_enospc") && !atomic_load(&fired)) {
        for (int i = 0; i < count; ++i) if (iov[i].iov_len) return write(fd, iov[i].iov_base, iov[i].iov_len);
    }
    long rc = syscall(SYS_writev, fd, iov, count);
    int error = rc < 0 ? errno : 0;
    if (scoped) event("staged_write", "writev", path, NULL, fd, 0, 0, rc, error, false, true, false);
    errno = error;
    return rc;
}

int fsync(int fd) {
    char path[PATH_MAX];
    bool resolved = enabled && fd_path(fd, path);
    bool temporary = resolved && own_temp(path);
    bool directory = resolved && !strcmp(path, root_path);
    const char *stage = temporary ? "file_sync" : "directory_sync";
    bool fail = temporary && !strcmp(fault_mode, "file_sync_eio");
    fail = fail || (directory && atomic_load(&renamed) && !strcmp(fault_mode, "dir_sync_eio"));
    if ((temporary || directory) && armed() && fail && take_fault()) {
        event(stage, "fsync", path, NULL, fd, 0, 0, -1, EIO, true, false, atomic_load(&renamed));
        errno = EIO;
        return -1;
    }
    long rc = syscall(SYS_fsync, fd);
    int error = rc < 0 ? errno : 0;
    if (temporary || directory) event(stage, "fsync", path, NULL, fd, 0, 0, rc, error, false, true, atomic_load(&renamed));
    errno = error;
    return (int)rc;
}

static int perform_rename(int oldfd, const char *oldpath, int newfd, const char *newpath) {
    bool scoped = own_temp(oldpath) && newpath && !strcmp(newpath, target_path);
    if (scoped && armed() && !strcmp(fault_mode, "rename_before") && take_fault()) {
        event("install_rename", "rename", oldpath, newpath, -1, 0, 0, -1, EIO, true, false, false);
        errno = EIO;
        return -1;
    }
    long rc = syscall(SYS_renameat, oldfd, oldpath, newfd, newpath);
    int error = rc < 0 ? errno : 0;
    if (scoped) {
        if (rc == 0 && armed()) atomic_store(&renamed, 1);
        event("install_rename", "rename", oldpath, newpath, -1, 0, 0, rc, error, false, true, rc == 0);
        if (rc == 0 && armed() && !strcmp(fault_mode, "rename_after") && take_fault()) {
            event("install_rename", "rename_return", oldpath, newpath, -1, 0, 0, -1, EIO, true, false, true);
            errno = EIO;
            return -1;
        }
    }
    errno = error;
    return (int)rc;
}
int rename(const char *oldpath, const char *newpath) { return perform_rename(AT_FDCWD, oldpath, AT_FDCWD, newpath); }
int renameat(int oldfd, const char *oldpath, int newfd, const char *newpath) { return perform_rename(oldfd, oldpath, newfd, newpath); }

static int perform_link(int oldfd, const char *oldpath, int newfd, const char *newpath, int flags) {
    bool scoped = own_temp(oldpath) && newpath && !strcmp(newpath, target_path);
    long rc = syscall(SYS_linkat, oldfd, oldpath, newfd, newpath, flags);
    int error = rc < 0 ? errno : 0;
    if (scoped) {
        if (rc == 0 && armed()) { snprintf(linked_source, sizeof linked_source, "%s", oldpath); atomic_store(&linked, 1); }
        event("install_link", "linkat", oldpath, newpath, -1, 0, 0, rc, error, false, true, rc == 0);
    }
    errno = error;
    return (int)rc;
}
int link(const char *oldpath, const char *newpath) { return perform_link(AT_FDCWD, oldpath, AT_FDCWD, newpath, 0); }
int linkat(int oldfd, const char *oldpath, int newfd, const char *newpath, int flags) { return perform_link(oldfd, oldpath, newfd, newpath, flags); }

static int perform_unlink(int dirfd, const char *path, int flags) {
    bool scoped = own_temp(path);
    if (scoped && armed() && !strcmp(fault_mode, "cleanup_unlink") && atomic_load(&linked)
        && !strcmp(path, linked_source) && take_fault()) {
        event("staging_cleanup", "unlink", path, target_path, -1, 0, 0, -1, EIO, true, false, true);
        errno = EIO;
        return -1;
    }
    long rc = syscall(SYS_unlinkat, dirfd, path, flags);
    int error = rc < 0 ? errno : 0;
    if (scoped) event("staging_cleanup", "unlink", path, NULL, -1, 0, 0, rc, error, false, true, false);
    errno = error;
    return (int)rc;
}
int unlink(const char *path) { return perform_unlink(AT_FDCWD, path, 0); }
int unlinkat(int dirfd, const char *path, int flags) { return perform_unlink(dirfd, path, flags); }
