/* eslint-disable no-console */
import "spectre.css/src/spectre.scss";
import "fast-text-encoding";
import * as Sentry from "@sentry/browser";
import { register } from "register-service-worker";

Sentry.init({
  dsn: "https://046b94f8170f4135a47ca9d0f9709a6d@sentry.io/1438468",
});

async function startup(): Promise<void> {
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  // eslint-disable-next-line import/no-unresolved
  await import("../pkg");
}

void startup();

register("/service-worker.js", {
  ready(): void {
    console.log("Service worker is active.");
  },
  registered(): void {
    console.log("Service worker has been registered.");
  },
  cached(): void {
    console.log("Content has been cached for offline use.");
  },
  updatefound(): void {
    console.log("New content is downloading.");
  },
  updated(): void {
    console.log("New content is available; please refresh.");
  },
  offline(): void {
    console.log(
      "No internet connection found. App is running in offline mode."
    );
  },
  error(error): void {
    Sentry.captureException(error);
    console.error("Error during service worker registration:", error);
  },
});
