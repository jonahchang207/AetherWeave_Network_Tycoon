(function () {
  const toggle = document.querySelector(".nav-toggle");
  const nav = document.querySelector(".site-nav");
  if (toggle && nav) {
    toggle.addEventListener("click", () => {
      const open = nav.classList.toggle("open");
      toggle.setAttribute("aria-expanded", open ? "true" : "false");
    });
  }

  const tabBars = document.querySelectorAll("[data-tab-group]");
  tabBars.forEach((bar) => {
    const group = bar.getAttribute("data-tab-group");
    const buttons = bar.querySelectorAll("button[data-tab]");
    const panels = document.querySelectorAll(`[data-tab-panel="${group}"]`);

    function activate(tabId) {
      buttons.forEach((btn) => {
        btn.classList.toggle("active", btn.getAttribute("data-tab") === tabId);
      });
      panels.forEach((panel) => {
        panel.classList.toggle("active", panel.getAttribute("data-tab-id") === tabId);
      });
    }

    buttons.forEach((btn) => {
      btn.addEventListener("click", () => activate(btn.getAttribute("data-tab")));
    });

    if (buttons.length) {
      activate(buttons[0].getAttribute("data-tab"));
    }
  });

  const path = window.location.pathname;
  document.querySelectorAll(".doc-sidebar a").forEach((link) => {
    if (link.getAttribute("href") && path.endsWith(link.getAttribute("href").replace(/\/$/, ""))) {
      link.classList.add("active");
    } else if (link.pathname === path) {
      link.classList.add("active");
    }
  });
})();
