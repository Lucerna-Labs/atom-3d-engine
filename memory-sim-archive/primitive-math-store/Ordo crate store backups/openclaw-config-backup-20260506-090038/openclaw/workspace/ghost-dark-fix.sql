-- Replace the code injection head with properly scoped dark theme CSS
-- Key fix: all styles scoped to .gh-site or body:not(.gh-admin) to avoid breaking Ghost admin
UPDATE settings SET value = '<style>
  /* Dark theme - scoped to frontend only, NOT Ghost admin */
  :root {
    --ghost-accent-color: #ffffff;
    --background-color: #000000;
    --foreground-color: #ffffff;
  }

  body.gh-site {
    background-color: #000000 !important;
    color: #ffffff !important;
  }

  .gh-site .gh-header {
    background-color: #000000 !important;
  }

  .gh-site .gh-header-inner {
    color: #ffffff !important;
  }

  .gh-site .gh-head-logo,
  .gh-site .gh-head-logo a {
    color: #ffffff !important;
  }

  .gh-site .gh-head-nav a,
  .gh-site .gh-head-menu a {
    color: #ffffff !important;
  }

  .gh-site .gh-footer {
    background-color: #000000 !important;
    color: #ffffff !important;
  }

  .gh-site a {
    color: #ffffff !important;
  }

  .gh-site h1,
  .gh-site h2,
  .gh-site h3,
  .gh-site h4,
  .gh-site h5,
  .gh-site h6,
  .gh-site p,
  .gh-site span,
  .gh-site li {
    color: #ffffff !important;
  }

  .gh-site .gh-card a {
    color: #ffffff !important;
  }

  .gh-site .gh-card .gh-card-excerpt {
    color: #cccccc !important;
  }

  .gh-site .gh-card .gh-card-meta {
    color: #999999 !important;
  }

  .gh-site .gh-hero {
    color: #ffffff !important;
  }

  .gh-site .gh-hero-title {
    color: #ffffff !important;
  }

  .gh-site .gh-hero-description {
    color: #cccccc !important;
  }

  .gh-site .gh-form {
    background: #111111 !important;
    border-color: #333333 !important;
  }

  .gh-site .gh-form input {
    background: #111111 !important;
    color: #ffffff !important;
    border-color: #333333 !important;
  }

  .gh-site .gh-btn-primary {
    background: #ffffff !important;
    color: #000000 !important;
  }

  .gh-site .gh-btn-primary:hover {
    background: #cccccc !important;
  }

  .gh-site .gh-btn {
    background: #ffffff !important;
    color: #000000 !important;
  }

  .gh-site .gh-btn:hover {
    background: #cccccc !important;
  }

  .gh-site .gh-content {
    color: #ffffff !important;
  }

  .gh-site .gh-content p,
  .gh-site .gh-content li,
  .gh-site .gh-content blockquote {
    color: #ffffff !important;
  }

  .gh-site .gh-feature-image {
    border-radius: 8px;
  }
</style>' WHERE `key` = 'codeinjection_head';

SELECT `key`, LEFT(value, 100) as preview FROM settings WHERE `key` = 'codeinjection_head';