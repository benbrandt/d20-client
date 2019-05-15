import "bulma/bulma.sass";
import * as Sentry from "@sentry/browser";
import { register } from "register-service-worker";

Sentry.init({
  dsn: "https://046b94f8170f4135a47ca9d0f9709a6d@sentry.io/1438468"
});

// @ts-ignore
// eslint-disable-next-line import/no-unresolved
import("../pkg").then(
  (module): void => {
    module.render();
  }
);

register("/service-worker.js", {
  ready() {
    console.log("Service worker is active.");
  },
  registered() {
    console.log("Service worker has been registered.");
  },
  cached() {
    console.log("Content has been cached for offline use.");
  },
  updatefound() {
    console.log("New content is downloading.");
  },
  updated() {
    console.log("New content is available; please refresh.");
  },
  offline() {
    console.log(
      "No internet connection found. App is running in offline mode."
    );
  },
  error(error) {
    Sentry.captureException(error);
    console.error("Error during service worker registration:", error);
  }
});
