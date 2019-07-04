/* eslint-disable no-console */
import "spectre.css/src/spectre.scss";
import "fast-text-encoding";
import createAuth0Client from "@auth0/auth0-spa-js";
import Auth0Client from "@auth0/auth0-spa-js/dist/typings/Auth0Client";
import * as Sentry from "@sentry/browser";
import { register } from "register-service-worker";

Sentry.init({
  dsn: "https://046b94f8170f4135a47ca9d0f9709a6d@sentry.io/1438468"
});

/**
 * Fire event that can be handled by Seed application
 *
 * @param {string} msgName Msg variant name, e.g. Tick(String) will be "Tick"
 * @param {*|undefined} data Serialized Msg variant data
 */
function triggerUpdate(msgName: string, msgData?: unknown): void {
  const event = new CustomEvent("triggerupdate", {
    detail: msgData === undefined ? msgName : { [msgName]: msgData }
  });
  window.dispatchEvent(event);
}

let auth0: Auth0Client;

async function checkAuth(): Promise<boolean> {
  const isAuthenticated = await auth0.isAuthenticated();
  triggerUpdate("Authenticated", isAuthenticated);
  //if (isAuthenticated) {
  //  const token = await auth0.getTokenSilently();
  //  const user = await auth0.getUser();
  //}
  return isAuthenticated;
}

async function authenticate(): Promise<void> {
  auth0 = await createAuth0Client({
    domain: "d20.auth0.com",
    client_id: "IjRu7XqVRtUIEOjwhzC7Bbe2P1zRVhPC"
  });

  const isAuthenticated = await checkAuth();
  if (isAuthenticated) return;

  const query = window.location.search;
  if (query.includes("code=") && query.includes("state=")) {
    // Process the login state
    await auth0.handleRedirectCallback();
    await checkAuth();
    // Use replaceState to redirect the user away and remove the querystring parameters
    window.history.replaceState({}, document.title, "/");
  }
}

async function login(): Promise<void> {
  await auth0.loginWithRedirect({
    redirect_uri: window.location.origin
  });
}

function logout(): void {
  auth0.logout({
    returnTo: window.location.origin
  });
}

// @ts-ignore
window.d20Login = login;
// @ts-ignore
window.d20Logout = logout;

async function startup(): Promise<void> {
  // @ts-ignore
  // eslint-disable-next-line import/no-unresolved
  const module = await import("../pkg");
  module.render();
  await authenticate();
}

startup();

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
  }
});
