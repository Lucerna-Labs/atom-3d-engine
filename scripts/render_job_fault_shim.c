/* Linux-only acceptance interposer, never linked into the editor. Kill the real
 * process at a selected local job-storage boundary after writing trigger evidence. */
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/syscall.h>
#include <unistd.h>
#include <limits.h>

static const char *job_root(void) { return getenv("MM3E_JOB_FAULT_ROOT"); }
static const char *mode(void) { const char *m=getenv("MM3E_JOB_FAULT_MODE");return m?m:"none"; }
static int target(const char *path,const char *name) {
    const char *root=job_root();if(!root)return 0;
    char expected[PATH_MAX];if(snprintf(expected,sizeof expected,"%s/%s",root,name)>=(int)sizeof expected)return 0;
    return !strcmp(path,expected);
}
static int contains(const char *path,const char *needle) {
    int fd=open(path,O_RDONLY|O_CLOEXEC);if(fd<0)return 0;
    char buffer[262144];ssize_t n=read(fd,buffer,sizeof buffer-1);close(fd);
    if(n<0)return 0;
    buffer[n]=0;
    return strstr(buffer,needle)!=NULL;
}
static void die(const char *stage) {
    const char *root=job_root();char path[PATH_MAX],event[256];
    if(!root || snprintf(path,sizeof path,"%s/fault-trigger.json",root)>=(int)sizeof path)_exit(93);
    int fd=open(path,O_WRONLY|O_CREAT|O_EXCL|O_CLOEXEC,0600);
    if(fd<0)_exit(94);
    int n=snprintf(event,sizeof event,"{\"mode\":\"%s\",\"stage\":\"%s\",\"pid\":%ld}\n",mode(),stage,(long)getpid());
    if(write(fd,event,(size_t)n)!=n || fsync(fd))_exit(95);
    close(fd);
    _exit(86);
}
static int link_impl(int oldfd,const char *oldpath,int newfd,const char *newpath,int flags) {
    int frame=target(newpath,"frame_0000.png");
    if(frame && !strcmp(mode(),"frame_before"))die("pending durable; before frame link");
    long rc=syscall(SYS_linkat,oldfd,oldpath,newfd,newpath,flags);int saved=errno;
    if(!rc) {
        if(frame && !strcmp(mode(),"frame_after"))die("frame linked; pending still durable");
        if(target(newpath,"audio.wav") && !strcmp(mode(),"audio_after"))die("audio linked; manifest absent");
        if(target(newpath,"manifest.json") && !strcmp(mode(),"manifest_after"))die("manifest linked; job completion not acknowledged");
    }errno=saved;return (int)rc;
}
int link(const char *oldpath,const char *newpath){return link_impl(AT_FDCWD,oldpath,AT_FDCWD,newpath,0);}
int linkat(int oldfd,const char *oldpath,int newfd,const char *newpath,int flags){return link_impl(oldfd,oldpath,newfd,newpath,flags);}
static int rename_impl(int oldfd,const char *oldpath,int newfd,const char *newpath) {
    int selected=target(newpath,"state.json");
    int pending=selected && contains(oldpath,"\"pending\": {");
    int complete=selected && contains(oldpath,"\"completed_manifest\": {");
    if(pending && !strcmp(mode(),"pending_before"))die("image staged; before pending intent commit");
    long rc=syscall(SYS_renameat,oldfd,oldpath,newfd,newpath);int saved=errno;
    if(!rc) {
        if(pending && !strcmp(mode(),"pending_after"))die("pending intent committed; frame absent");
        if(complete && !strcmp(mode(),"complete_after"))die("completion state committed; response absent");
    }errno=saved;return (int)rc;
}
int rename(const char *oldpath,const char *newpath){return rename_impl(AT_FDCWD,oldpath,AT_FDCWD,newpath);}
int renameat(int oldfd,const char *oldpath,int newfd,const char *newpath){return rename_impl(oldfd,oldpath,newfd,newpath);}
