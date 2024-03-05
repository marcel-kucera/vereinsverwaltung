import { login, logout, type Login, getLoginStatus } from "$lib/api/login";
import { emptyPromise } from "$lib/util";

let loginStatus: Promise<boolean> = $state(emptyPromise());
let loginPromise: Promise<Response> | undefined = $state(undefined);
let logoutPromise: Promise<Response> | undefined = $state(undefined);

let checkStatus = () => (loginStatus = getLoginStatus());

export const authStore = {
  get loginStatus() {
    return loginStatus;
  },
  get loginPromise() {
    return loginPromise;
  },
  get logoutPromise() {
    return logoutPromise;
  },
  checkStatus,
  login(data: Login) {
    loginPromise = login(data);
    logoutPromise = undefined;
    loginPromise.then(checkStatus);
  },
  logout() {
    logoutPromise = logout();
    loginPromise = undefined;
    logoutPromise.then(checkStatus);
  },
};
