// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  sidebar: [
    {
      type: "category",
      label: "Installation",
      items: [
        "installation/docker",
        "installation/cargo",
        "installation/from-source",
      ],
      collapsed: false,
    },
    {
      type: "category",
      label: "Usage",
      items: ["usage/arguments", "usage/config"],
      collapsed: true,
    },
    {
      type: "category",
      label: "Modules",
      items: [
        "modules/domain_takeover",
        "modules/dork",
        "modules/passive_dns",
        "modules/screenshot_grabber",
      ],
      collapsed: true,
    },
    {
      type: "category",
      label: "Scripting",
      items: ["scripting/basics"],
      collapsed: true,
    },
  ],
};

module.exports = sidebars;
