UPDATE settings SET value = '<script>
// Add Bluesky and Mastodon social links to Ghost footer since Ghost has no native support
document.addEventListener("DOMContentLoaded", function() {
  var socialNav = document.querySelector(".gh-head-menu, .gh-footer-menu, nav.gh-social");
  if (socialNav) {
    var bsky = document.createElement("a");
    bsky.href = "https://bsky.app/profile/warped-reality.bsky.social";
    bsky.title = "Bluesky";
    bsky.target = "_blank";
    bsky.rel = "noopener";
    bsky.innerHTML = \'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M12 10.8c-1.087-2.114-4.045-5.706-6.808-5.706-3.028 0-4.792 2.108-4.792 4.583 0 2.475 2.4 5.706 5.428 5.706 2.763 0 5.345-2.975 6.172-4.583.827 1.608 3.41 4.583 6.172 4.583 3.028 0 5.428-3.231 5.428-5.706 0-2.475-1.764-4.583-4.792-4.583-2.763 0-5.72 3.592-6.808 5.706z"/></svg> Bluesky\';
    socialNav.appendChild(bsky);
    var masto = document.createElement("a");
    masto.href = "https://mastodon.social/@Warped_Reality";
    masto.title = "Mastodon";
    masto.target = "_blank";
    masto.rel = "me noopener";
    masto.innerHTML = \'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M23.268 5.313c-.35-2.578-2.157-4.94-4.93-5.596C14.3-.82 8.018-.82 3.974.9 1.65 1.878.468 3.632.248 5.313.03 6.98-.06 8.748.03 10.5c.05.94.2 1.92.46 2.86.82 2.96 2.71 5.37 5.42 6.68 2.72 1.32 5.84 1.64 8.82.95 2.98-.7 5.57-2.56 7.17-5.11.97-1.55 1.57-3.32 1.77-5.15.3-2.75.2-5.55-.17-8.28-.04-.3-.1-.6-.16-.9zM20.56 9.87c-.06 1.56-.47 3.12-1.2 4.48-1.23 2.3-3.44 3.95-5.97 4.37-2.53.42-5.16-.24-7.17-1.85-2-1.6-3.26-4.02-3.42-6.63-.1-1.7-.04-3.4.16-5.08.1-.85.4-1.67.86-2.38.75-1.14 1.9-1.97 3.2-2.3 2.6-.67 5.4-.67 8 0 1.3.34 2.44 1.16 3.2 2.3.46.7.76 1.53.86 2.38.2 1.5.3 3.13.2 4.7z"/></svg> Mastodon\';
    socialNav.appendChild(masto);
  }
});
</script>' WHERE `key` = 'codeinjection_foot';
SELECT `key`, LEFT(value, 80) as preview FROM settings WHERE `key` = 'codeinjection_foot';