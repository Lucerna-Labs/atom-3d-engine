-- Update code injection head for dark theme
UPDATE settings SET value='<style>
  :root {
    --ghost-accent-color: #ffffff;
  }
  body, .gh-canvas, .gh-outer, .gh-inner {
    background-color: #000000 !important;
    color: #ffffff !important;
  }
  .gh-header {
    background-color: #000000 !important;
  }
  .gh-header-inner {
    color: #ffffff !important;
  }
  .gh-head-logo {
    color: #ffffff !important;
  }
  .gh-head-nav a, .gh-head-menu a {
    color: #ffffff !important;
  }
  .gh-footer {
    background-color: #000000 !important;
    color: #ffffff !important;
  }
  a {
    color: #ffffff !important;
  }
  h1, h2, h3, h4, h5, h6, p, span, li {
    color: #ffffff !important;
  }
  .gh-card a {
    color: #ffffff !important;
  }
  .gh-card .gh-card-excerpt {
    color: #cccccc !important;
  }
  .gh-card .gh-card-meta {
    color: #999999 !important;
  }
  /* Landing page specific */
  .gh-home-template .gh-hero {
    color: #ffffff !important;
  }
  .gh-home-template .gh-hero-title {
    color: #ffffff !important;
  }
  .gh-home-template .gh-hero-description {
    color: #cccccc !important;
  }
  /* Subscribe/form elements */
  .gh-form {
    background: #111111 !important;
    border-color: #333333 !important;
  }
  .gh-form input {
    background: #111111 !important;
    color: #ffffff !important;
    border-color: #333333 !important;
  }
  .gh-btn {
    background: #ffffff !important;
    color: #000000 !important;
  }
  .gh-btn:hover {
    background: #cccccc !important;
  }
  /* Post content */
  .gh-content {
    color: #ffffff !important;
  }
  .gh-content p, .gh-content li, .gh-content blockquote {
    color: #ffffff !important;
  }
  /* Feature image overlay for dark theme */
  .gh-feature-image {
    border-radius: 8px;
  }
</style>' WHERE `key` = 'codeinjection_head';
SELECT `key`, LEFT(value, 50) as value_preview FROM settings WHERE `key` = 'codeinjection_head';