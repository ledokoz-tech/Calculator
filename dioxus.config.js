/**
 * @type {import("dioxus-cli").DioxusConfig}
 */
module.exports = {
  /**
   * The path to the web assembly bundle and html file.
   * @type {import("dioxus-cli").BuildAssets}
   */
  app: {
    static_dir: "public", // Directory for static assets like CSS and images
    index_file: "index.html", // The main HTML file for your Dioxus app
  },
  // Other Dioxus CLI configurations can go here
  // For example, you can configure features, plugins, etc.
};
