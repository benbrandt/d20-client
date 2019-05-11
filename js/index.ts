import "bulma/bulma.sass";
import * as Sentry from "@sentry/browser";

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
