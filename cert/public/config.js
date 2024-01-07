function add(data) {
  if (data && data.meta) {
    const rdMeta = document.createElement("meta");

    const metaKeys = Object.keys(data.meta);
    for (const metaKey of metaKeys) {
      const metaValue = data.meta[metaKey];
      if (metaValue) rdMeta.setAttribute(metaKey, metaValue);
    }
    document.head.appendChild(rdMeta);
  } else if (data && data.title) {
    const rdTitle = document.createElement('title')

    rdTitle.innerHTML = data.title;
    document.head.appendChild(rdTitle);
  }
}

if (localStorage) {
  const configRaw = localStorage.getItem("config");
  if (configRaw) {
    try {
      const config = JSON.parse(configRaw);
      if (config) {
        if (config.color.primary)
          document.documentElement.style.setProperty(
            "--primary-color",
            `rgb(${config.color.primary.join(",")})`
          );
        if (config.color.secondary)
          document.documentElement.style.setProperty(
            "--secondary-color",
            `rgb(${config.color.secondary.join(",")})`
          );
        if (config.color.error)
          document.documentElement.style.setProperty(
            "--error-color",
            `rgb(${config.color.error.join(",")})`
          );
        if (config.color.warning)
          document.documentElement.style.setProperty(
            "--warning-color",
            `rgb(${config.color.warning.join(",")})`
          );
        if (config.color.success)
          document.documentElement.style.setProperty(
            "--success-color",
            `rgb(${config.color.success.join(",")})`
          );
        if (config.color.font_primary)
          document.documentElement.style.setProperty(
            "--font-primary-color",
            `rgb(${config.color.font_primary.join(",")})`
          );
        if (config.color.font_secondary)
          document.documentElement.style.setProperty(
            "--font-secondary-color",
            `rgb(${config.color.font_secondary.join(",")})`
          );
        if (config.color.background_one)
          document.documentElement.style.setProperty(
            "--background-depth-one-color",
            `rgb(${config.color.background_one.join(",")})`
          );
        if (config.color.background_two)
          document.documentElement.style.setProperty(
            "--background-depth-two-color",
            `rgb(${config.color.background_two.join(",")})`
          );
        if (config.color.background_three)
          document.documentElement.style.setProperty(
            "--background-depth-three-color",
            `rgb(${config.color.background_three.join(",")})`
          );

        add({
          meta: {
            property: "og:type",
            content: "website",
          }
        });
        add({
          meta: {
            property: "og:url",
            content: window.location.origin,
          }
        });

        if (config.description) {
          add({
            meta: {
              name: "description",
              content: config.description,
            }
          });
          add({
            meta: {
              property: "og:description",
              content: config.description,
            }
          });
        }
        if (config.title) {
          let name = config.title;
          if (config.name) {
            name = `${config.name}: ${config.title}`;
          }
          add({
            meta: {
              name: "og:title",
              content: name,
            }
          });
          add({
            title: name
          })
        }
      }
    } catch { }
  }
}
