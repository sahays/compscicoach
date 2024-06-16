const path = require("path");
const glob = require("glob");

const scripts = glob
	.sync("./assets/scripts/**/*.js")
	.map((file) => "./" + file);
console.log(scripts);

module.exports = {
	entry: scripts, // Match all JS files in the assets/scripts folder
	output: {
		filename: "bundle.js",
		path: path.resolve(__dirname, "dist"),
	},
	mode: "development",
	watch: true, // Enable watch mode
	watchOptions: {
		aggregateTimeout: 300, // Delay the rebuild after the first change (in milliseconds)
		poll: 1000, // Check for changes every second (in milliseconds)
		ignored: /node_modules/, // Ignore node_modules folder
	},
	module: {
		rules: [
			{
				test: /\.m?js$/,
				exclude: /(node_modules|bower_components)/,
				use: {
					loader: "babel-loader",
					options: {
						presets: ["@babel/preset-env"],
					},
				},
			},
		],
	},
};
