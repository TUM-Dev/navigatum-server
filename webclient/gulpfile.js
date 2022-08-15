/* eslint func-names: ["error", "always"] */

import gulp from "gulp";
import addsrc from "gulp-add-src";
import babel from "gulp-babel";
import concat from "gulp-concat";
import csso from "gulp-csso";
import first from "gulp-first";
import htmlmin from "gulp-htmlmin";
import i18n from "gulp-html-i18n";
import i18nCompile from "gulp-i18n-compile";
import gulpif from "gulp-if";
import inject from "gulp-inject";
import injectStr from "gulp-inject-string";
import injectHtml from "gulp-inject-stringified-html";
import markdown, { marked } from "gulp-markdown";
import postcss from "gulp-postcss";
import preprocess from "gulp-preprocess";
import purgecss from "gulp-purgecss";
import rename from "gulp-rename";
import revAll from "gulp-rev-all";
import _sass from "gulp-sass";
import sitemap from "gulp-sitemap";
import splitFiles from "gulp-split-files";
import uglify from "gulp-uglify";
import yaml from "gulp-yaml";

import browserify from "browserify";
import del from "delete";
import fs from "fs";
import merge from "merge-stream";
import dartSass from "sass";
import path from "path";
import postcssRemoveDeclaration from "postcss-remove-declaration";
import postcssPrependSelector from "postcss-prepend-selector";
import source from "vinyl-source-stream";

import { configRelease, configLocal } from "./config.js"; // eslint-disable-line import/extensions

// from https://github.com/gulpjs/gulp/blob/master/docs/recipes/running-task-steps-per-folder.md
function getFolders(dir) {
  return fs
    .readdirSync(dir)
    .filter((file) => fs.statSync(path.join(dir, file)).isDirectory());
}

// Selected at the bottom of the script
let config;

const sass = _sass(dartSass); // Select Sass compiler

const htmlminOptions = {
  caseSensitive: false,
  collapseBooleanAttributes: true,
  collapseInlineTagWhitespace: false,
  collapseWhitespace: true,
  conservativeCollapse: false,
  html5: true,
  includeAutoGeneratedTags: false,
  keepClosingSlash: false,
  minifyCSS: true,
  minifyJS: true,
  preserveLineBreaks: false,
  preventAttributesEscaping: false,
  // processConditionalComments: true,
  removeAttributeQuotes: true,
  removeComments: true,
  removeEmptyAttributes: true,
  removeOptionalTags: false, //!
  removeScriptTypeAttributes: true,
  removeStyleLinkTypeAttributes: true,
  sortAttributes: true,
  // sortClassName: true
};

const i18nOptions = {
  langDir: "build/tmp/locale",
};

const babelTargets = {
  browsers: ["last 2 versions", "not dead", "> 0.2%"],
};

// --- Preparations ---
function cleanBuild(cb) {
  del(["build/**"], cb);
}

// --- Main CSS Pipeline ---
function compileMainScss() {
  return merge(
    ["light", "dark"].map((theme) =>
      gulp
        .src("src/main.scss")
        .pipe(injectStr.prepend(`$theme: "${theme}";\n`))
        .pipe(sass().on("error", sass.logError))
        .pipe(rename(`main-${theme}.css`))
        .pipe(gulp.dest("build/tmp"))
    )
  );
}

function compileSpectreScss() {
  return merge(
    ["light", "dark"].map((theme) =>
      gulp
        .src("src/spectre-all.scss")
        .pipe(injectStr.prepend(`$theme: "${theme}";\n`))
        .pipe(sass().on("error", sass.logError))
        .pipe(
          purgecss({
            content: ["src/index.html", "src/views/*/*.inc"],
          })
        )
        // .pipe(csso())
        .pipe(rename(`spectre-all-purged-${theme}.css`))
        .pipe(gulp.dest("build/tmp"))
    )
  );
}

/* function mergeMainCss() {
  return merge(["light", "dark"].map((theme) =>
      gulp
        .src([
          `build/tmp/main-${theme}.css`,
          `build/tmp/spectre-all-purged-${theme}.css`,
        ])
        .pipe(concat(`app-main-merged-${theme}.css`))
        .pipe(gulp.dest("build/css"))
        .pipe(csso())
        .pipe(rename(`app-main-merged-${theme}.min.css`))
        .pipe(gulp.dest("build/css"))
    )
  );
} */
gulp.task(
  "main_css",
  gulp.series(compileMainScss, compileSpectreScss /* , mergeMainCss */)
);

// --- Main JS Pipeline ---
function buildAppCoreJS() {
  return gulp
    .src(["src/core.js", "src/detect-webp.js"])
    .pipe(concat("app-core.js"))
    .pipe(
      preprocess({
        context: {
          app_prefix: config.app_prefix,
          cdn_prefix: config.cdn_prefix,
          api_prefix: config.api_prefix,
          target: config.target,
        },
        includeBase: "src/js",
      })
    )
    .pipe(gulp.dest("build/js"));
}

function buildAppRestJS() {
  return gulp
    .src([
      "src/modules/interactive-map.js",
      "src/modules/autocomplete.js",
      // History states are here because usually this module is not needed
      // very soon, and if it is still missing this is not a real issue.
      "src/history-states.js",
    ])
    .pipe(concat("app-rest.js"))
    .pipe(
      preprocess({
        context: {
          app_prefix: config.app_prefix,
          cdn_prefix: config.cdn_prefix,
          api_prefix: config.api_prefix,
          target: config.target,
        },
        includeBase: "src/js",
      })
    )
    .pipe(gulp.dest("build/js"));
}

function buildFeedbackJS() {
  return gulp
    .src("src/feedback.js")
    .pipe(i18n(i18nOptions))
    .pipe(
      preprocess({
        context: {
          app_prefix: config.app_prefix,
          cdn_prefix: config.cdn_prefix,
          api_prefix: config.api_prefix,
          target: config.target,
        },
        includeBase: "src/js",
      })
    )
    .pipe(gulpif(config.target === "release", uglify()))
    .pipe(gulpif(config.target === "release", rename({ suffix: ".min" })))
    .pipe(gulp.dest("build/js"));
}

function copyVueJS() {
  if (config.target === "release")
    return gulp
      .src([
        "node_modules/vue/dist/vue.min.js",
        "node_modules/vue-router/dist/vue-router.min.js",
        "src/init-call.js",
      ])
      .pipe(concat("vue.min.js"))
      .pipe(gulp.dest("build/js"));
  return gulp
    .src([
      "node_modules/vue/dist/vue.js",
      "node_modules/vue-router/dist/vue-router.js",
      "src/init-call.js",
    ])
    .pipe(concat("vue.js"))
    .pipe(gulp.dest("build/js"));
}

gulp.task(
  "main_js",
  gulp.series(buildAppCoreJS, buildAppRestJS, buildFeedbackJS, copyVueJS)
);

// --- Views compilation pipeline ---
gulp.task("views", (done) => {
  const viewsSrcPath = "src/views";

  const folders = getFolders(viewsSrcPath);
  if (folders.length === 0) return done(); // nothing to do!

  const tasks = folders.map((folder) => {
    const cssTask = merge(
      ["light", "dark"].map((theme) =>
        gulp
          .src(path.join(viewsSrcPath, folder, `/view-${folder}.scss`))
          .pipe(injectStr.prepend(`$theme: "${theme}";\n`))
          .pipe(sass().on("error", sass.logError))
          .pipe(rename(`view-${theme}.css`))
          .pipe(gulp.dest(`build/tmp/views/${folder}`))
      )
    );

    const jsTask = gulp
      .src(path.join(viewsSrcPath, folder, `/view-${folder}.js`))
      .pipe(
        preprocess({
          context: {
            app_prefix: config.app_prefix,
            cdn_prefix: config.cdn_prefix,
            api_prefix: config.api_prefix,
            target: config.target,
          },
          includeBase: path.join(viewsSrcPath, folder),
        })
      )
      .pipe(rename("view.js"))
      .pipe(gulp.dest(`build/tmp/views/${folder}`));

    const htmlTask = gulp
      .src(path.join(viewsSrcPath, folder, `/view-${folder}.inc`))
      .pipe(
        preprocess({
          context: {
            app_prefix: config.app_prefix,
            cdn_prefix: config.cdn_prefix,
            api_prefix: config.api_prefix,
            target: config.target,
          },
          includeBase: path.join(viewsSrcPath, folder),
        })
      )
      .pipe(htmlmin(htmlminOptions))
      .pipe(gulp.dest(`build/tmp/views/${folder}`));

    return merge(cssTask, jsTask, htmlTask);
  });

  return merge(tasks);
});

// --- Build pages sources ---
gulp.task("pages_src", (done) => {
  const viewsBuildPath = "build/tmp/views";

  const folders = getFolders(viewsBuildPath);
  if (folders.length === 0) return done(); // nothing to do!

  const tasks = folders.map((folder) => {
    const viewCSS = merge(
      ["light", "dark"].map((theme) => {
        // Extract used spectre classes for this view and merge with core & view css
        const viewCSSCore = gulp
          .src(`build/tmp/spectre-all-purged-${theme}.css`)
          .pipe(concat("view-spectre-used.css"))
          .pipe(
            purgecss({
              content: ["src/index.html", `src/views/${folder}/*.inc`],
            })
          )
          .pipe(
            addsrc([
              path.join(viewsBuildPath, folder, `view-${theme}.css`),
              `build/tmp/main-${theme}.css`,
            ])
          )
          .pipe(concat(`view-core-merged-${theme}.css`))
          .pipe(csso())
          .pipe(rename(`view-core-merged-${theme}.min.css`))
          .pipe(gulp.dest(`build/tmp/views/${folder}`));

        // Merge remaining views css (TODO: include spectre somewhere else?)
        const viewCSSRest = gulp
          .src(
            [
              `build/tmp/views/*/view-${theme}.css`,
              `build/tmp/spectre-all-purged-${theme}.css`,
            ],
            { ignore: path.join(viewsBuildPath, folder, `view-${theme}.css`) }
          )
          .pipe(concat(`view-rest-merged-${theme}.css`))
          .pipe(gulp.dest(`build/tmp/views/${folder}`));
        return merge(viewCSSCore, viewCSSRest);
      })
    );

    const viewJS = gulp
      .src(path.join(viewsBuildPath, folder, "view.js"))
      .pipe(injectHtml())
      .pipe(rename("view-inlined.js"))
      .pipe(gulp.dest(`build/tmp/views/${folder}`));

    return merge(viewCSS, viewJS);
  });

  return merge(tasks);
});

// --- Build pages output ---
gulp.task("pages_out", (done) => {
  const viewsBuildPath = "build/tmp/views";

  const folders = getFolders(viewsBuildPath);
  if (folders.length === 0) return done(); // nothing to do!

  const tasks = folders.map((folder) => {
    const themedTasks = merge(
      ["light", "dark"].map((theme) => {
        const viewHtml = gulp
          .src("src/index.html")
          .pipe(rename(`index-view-${folder}-${theme}.html`))
          .pipe(i18n(i18nOptions))
          .pipe(
            preprocess({
              context: {
                view: folder,
                theme: theme,
                app_prefix: config.app_prefix,
                cdn_prefix: config.cdn_prefix,
                api_prefix: config.api_prefix,
                target: config.target,
              },
              includeBase: path.join(viewsBuildPath, folder),
            })
          )
          .pipe(
            inject(
              gulp.src(
                path.join(
                  viewsBuildPath,
                  folder,
                  `view-core-merged-${theme}.min.css`
                )
              ),
              {
                starttag: "<!-- inject:core:{{ext}} -->",
                transform: (filePath, file) => file.contents.toString("utf8"),
                quiet: true,
                removeTags: true,
              }
            )
          )
          .pipe(gulpif(config.target === "release", htmlmin(htmlminOptions)))
          .pipe(gulp.dest("build"));

        const copyCSS = gulp
          .src(
            path.join(viewsBuildPath, folder, `view-rest-merged-${theme}.css`)
          )
          .pipe(csso())
          .pipe(rename(`view-${folder}-rest-${theme}.min.css`))
          .pipe(gulp.dest("build/css"));

        return merge(viewHtml, copyCSS);
      })
    );

    const copyJSCore = gulp
      .src([
        "build/js/app-core.js",
        path.join(viewsBuildPath, folder, "view-inlined.js"),
      ])
      .pipe(concat(`app-core-for-view-${folder}.js`))
      .pipe(i18n(i18nOptions))
      .pipe(
        babel({
          presets: [
            [
              "@babel/preset-env",
              {
                targets: babelTargets,
                useBuiltIns: false,
              },
            ],
          ],
        })
      )
      .pipe(gulpif(config.target === "release", uglify()))
      .pipe(gulpif(config.target === "release", rename({ suffix: ".min" })))
      .pipe(gulp.dest("build/js"));

    const copyJSRest = gulp
      .src(["build/js/app-rest.js", "build/tmp/views/*/view-inlined.js"], {
        ignore: path.join(viewsBuildPath, folder, "view-inlined.js"),
      })
      .pipe(concat(`app-rest-for-view-${folder}.js`))
      .pipe(i18n(i18nOptions))
      .pipe(babel())
      .pipe(gulpif(config.target === "release", uglify()))
      .pipe(gulpif(config.target === "release", rename({ suffix: ".min" })))
      .pipe(gulp.dest("build/js"));

    return merge(themedTasks, copyJSCore, copyJSRest);
  });

  return merge(tasks);
});

// --- Legacy JS Pipeline ---
function buildWebpPolyfills() {
  return gulp
    .src([
      "node_modules/webp-hero/dist-cjs/polyfills.js",
      "node_modules/webp-hero/dist-cjs/webp-hero.bundle.js",
    ])
    .pipe(concat("webp-hero.min.js"))
    .pipe(gulp.dest("build/js"));
}

function extractPolyfills() {
  return (
    gulp
      .src(["src/legacy.js", "build/js/app-core.js", "build/js/app-rest.js"])
      .pipe(
        preprocess({
          context: {
            app_prefix: config.app_prefix,
            cdn_prefix: config.cdn_prefix,
            api_prefix: config.api_prefix,
            target: config.target,
          },
          includeBase: "src/js",
        })
      )
      .pipe(concat("tmp-merged.js"))
      .pipe(
        babel({
          presets: [
            [
              "@babel/preset-env",
              {
                targets: babelTargets,
                useBuiltIns: "usage",
                corejs: "3.8",
              },
            ],
          ],
          sourceType: "module",
        })
      )
      .pipe(splitFiles())
      .pipe(first())
      // Add custom polyfills for missing browser (not ES) features
      .pipe(addsrc("node_modules/whatwg-fetch/dist/fetch.umd.js"))
      .pipe(concat("polyfills.js"))
      .pipe(gulp.dest("build/tmp"))
  );
}

function insertPolyfills() {
  const bundleStream = browserify("./build/tmp/polyfills.js").bundle();

  return bundleStream.pipe(source("polyfills.js")).pipe(gulp.dest("build/tmp"));
}

function minifyPolyfills() {
  return gulp
    .src(["build/tmp/polyfills.js"])
    .pipe(gulpif(config.target === "release", uglify()))
    .pipe(gulpif(config.target === "release", rename({ suffix: ".min" })))
    .pipe(gulp.dest("build/js"));
}

gulp.task(
  "legacy_js",
  gulp.parallel(
    buildWebpPolyfills,
    gulp.series(extractPolyfills, insertPolyfills, minifyPolyfills)
  )
);

// --- I18n Pipeline ---
function i18nCompileLangfiles() {
  return gulp
    .src(["src/i18n.yaml", "src/views/*/i18n-*.yaml"])
    .pipe(yaml())
    .pipe(i18nCompile("[locale]/_.json", { localePlaceholder: "[locale]" }))
    .pipe(gulp.dest("build/tmp/locale"));
}

// --- Markdown Pipeline ---
const renderer = {
  code: (code, infostring) =>
    `<pre class="code" data-lang="${infostring}"><code>${code}</code></pre>`,
  link: (href, title, text) => {
    if (href.startsWith("http"))
      return `<a href="${href}" target="_blank">${text}</a>`;
    return `<router-link to="${href}">${text}</router-link>`;
  },
};
marked.use({ renderer: renderer });

function compileMarkdown() {
  return gulp
    .src("src/md/*.md")
    .pipe(
      markdown({
        headerPrefix: "md-",
      })
    )
    .pipe(gulp.dest("build/pages"));
}
gulp.task("markdown", compileMarkdown);

// --- Asset Pipeline ---
function copyAssets() {
  return gulp.src("src/assets/**").pipe(gulp.dest("build/assets"));
}
gulp.task("assets", copyAssets);

// --- Revisioning Pipeline ---
function revisionAssets(done) {
  if (config.target !== "release") return done();
  return gulp
    .src(["build/index-*.html", "build/js/*.js", "build/assets/*"])
    .pipe(
      revAll.revision({
        // Currently .js only, because important css is inlined, and postloaded
        // css is deferred using preload, which revAll currently doesn't detect
        includeFilesInManifest: [".js", ".webp", ".svg", ".png", ".ico"],
        dontRenameFile: [".html"],
        transformFilename: (file, hash) => {
          const ext = path.extname(file.path);
          return `cache_${hash.substr(0, 8)}.${path.basename(
            file.path,
            ext
          )}${ext}`;
        },
      })
    )
    .pipe(gulp.dest("build"));
}
gulp.task("revision_assets", revisionAssets);

// --- Sitemap Pipeline ---
function generateSitemap() {
  return gulp
    .src(["src/md/*.md", "src/index.html"], { read: false })
    .pipe(
      rename((pathObj) => {
        if (pathObj.extname === ".md") {
          pathObj.dirname = "about";
          pathObj.extname = "";
        } else {
          pathObj.dirname = "";
        }
      })
    )
    .pipe(
      sitemap({
        siteUrl: "https://nav.tum.sexy/",
        fileName: "sitemap-webclient.xml",
        changefreq: "monthly",
      })
    )
    .pipe(gulp.dest("build"));
}
gulp.task("sitemap", generateSitemap);

// --- .well-known Pipeline ---
// see https://well-known.dev/sites/
function copyWellKnown() {
  return gulp
    .src([
      "src/.well-known/gpc.json", // we don't sell or share data
      "src/.well-known/security.txt",
    ]) // security-advice
    .pipe(gulp.dest("build/.well-known"));
}
function copyWellKnownRoot() {
  return gulp
    .src([
      "src/.well-known/robots.txt", // disallow potentially costly api requests
      "src/.well-known/googlebef9161f1176c5e0.html",
    ]) // google search console
    .pipe(gulp.dest("build"));
}
gulp.task("well_known", gulp.parallel(copyWellKnown, copyWellKnownRoot));

// --- map (currently mapbox) Pipeline ---
function copyMapCSS() {
  return gulp
    .src(["node_modules/mapbox-gl/dist/mapbox-gl.css"])
    .pipe(concat("mapbox.css"))
    .pipe(gulpif(config.target === "release", csso()))
    .pipe(gulpif(config.target === "release", rename({ suffix: ".min" })))
    .pipe(gulp.dest("build/css"));
}
function copyMapJS() {
  return gulp
    .src(["node_modules/mapbox-gl/dist/mapbox-gl.js"])
    .pipe(concat("mapbox.js"))
    .pipe(gulpif(config.target === "release", uglify()))
    .pipe(gulpif(config.target === "release", rename({ suffix: ".min" })))
    .pipe(gulp.dest("build/js"));
}
gulp.task("map", gulp.parallel(copyMapCSS, copyMapJS));

// --- api-visualiser (currently swagger-ui) Pipeline ---
function copyApiCSS() {
  // swagger-ui has its own loading button
  const loadingCSS = {
    ".swagger-ui .loading-container": "*",
    ".swagger-ui .loading-container .loading": "*",
    ".swagger-ui .loading-container .loading:before":"*",
    ".swagger-ui .loading-container .loading::before":"*",
    ".swagger-ui .loading-container .loading:after":"*",
    ".swagger-ui .loading-container .loading::after":"*",
  };
  return gulp
    .src("node_modules/swaggerdark/SwaggerDark.css")
    .pipe(postcss([
      postcssRemoveDeclaration({remove: loadingCSS}),
      postcssPrependSelector({ selector: "body.theme-dark #swagger-ui " })
    ]))
    .pipe(csso())
    .pipe(addsrc.prepend("node_modules/swagger-ui-dist/swagger-ui.css"))
    .pipe(postcss([
      postcssRemoveDeclaration({remove: loadingCSS}),
    ]))
    .pipe(concat("swagger-ui.min.css")) // swagger-ui is already minified => minifying here does not make sense
    .pipe(gulp.dest("build/css"));
}
function copyApiJS() {
  return gulp
    .src(["node_modules/swagger-ui-dist/swagger-ui-bundle.js"])
    .pipe(concat("swagger-ui.min.js")) // swagger-ui is already minified => minifying here does not make sense
    .pipe(gulp.dest("build/js"));
}
gulp.task("api", gulp.parallel(copyApiCSS, copyApiJS));

const _build = gulp.series(
  cleanBuild,
  i18nCompileLangfiles,
  gulp.parallel(
    "main_css",
    "main_js",
    "views",
    "assets",
    "well_known",
    "map",
    "api",
    "markdown",
    "sitemap"
  ),
  gulp.series("pages_src", "pages_out", "legacy_js", "revision_assets")
);

const build = gulp.series((done) => {
  config = configLocal;
  config.target = "develop";
  done();
}, _build);

const release = gulp.series((done) => {
  config = configRelease;
  config.target = "release";
  done();
}, _build);

export default build;
export { release };
