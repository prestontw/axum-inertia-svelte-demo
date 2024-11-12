import { type Users, type UserId } from "./shared_definitions";

export function usersRoute(_: Users) {
  return "/users";
}

export function userIdRoute(user: UserId) {
  let { id } = user;
  return `/users/${id}`;
}
