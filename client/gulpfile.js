
const gulp = require("gulp");
const del = require("del");
const webpack = require("webpack-stream");
const eslint = require("gulp-eslint");
const plumber = require("gulp-plumber");
const notify = require("gulp-notify");

const webpackConfig = require("./webpack.config.js");

function clean() {
    return del("./public/dist");
}

function build() {
    return gulp.src([
            "src/index.tsx",
        ])
        .pipe(plumber({
            errorHandler: (err) => {
                notify.onError({
                    title: "Gulp",
                    message: "Error: <%= error.message %>"
                })(err);
                this.emit("end");
            }
        }))
        .pipe(eslint({ useEslintrc: true }))
        .pipe(eslint.format())
        .pipe(eslint.failOnError())
        .pipe(webpack(webpackConfig))
        .pipe(gulp.dest("./public/dist"));
}

exports.default = gulp.series([clean, build]);
