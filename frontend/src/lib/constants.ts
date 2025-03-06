import HandlersLesson from "../pages/HandlersLesson";
import RouterLesson from "../pages/RouterLesson";
import { type NavLink, Request } from "./types";
import axios from "axios";

export const apiUrl = "http://localhost:3000";

export const axiosClient = axios.create({
   baseURL: apiUrl,
   timeout: 10000,
   headers: {
      "Content-Type": "application/json",
   },
});

export const navLinks: NavLink[] = [
   {
      name: "Routers",
      to: "/",
      page: RouterLesson,
   },
   {
      name: "Handlers",
      to: "/handlers",
      page: HandlersLesson,
   },
];

export const requests = {
   routerRequests: [
      {
         url: "/connect/yabba/dabab/dopped",
         method: "GET",
         headers: {},
         body: "",
      },
      {
         url: "/crazy",
         method: "GET",
         headers: {},
         body: "",
      },
      {
         url: "/docs/structs",
         method: "GET",
         headers: {},
         body: "",
      },
      {
         url: "/docs/structs",
         method: "PUT",
         headers: {},
         body: "",
      },
      {
         url: "/foo",
         method: "PUT",
         headers: {},
         body: "",
      },
      {
         url: "/assets/hood.html",
         method: "GET",
         headers: {},
         body: "",
      },
      {
         url: "/assets/index.html",
         method: "GET",
         headers: {},
         body: "",
      },
      {
         url: "/assets/img.html",
         method: "GET",
         headers: {},
         body: "",
      },
      {
         url: "/api/9/asset/some/nice/image.png.minified",
         method: "GET",
         headers: {},
         body: "",
      },
      {
         url: "/",
         method: "GET",
         headers: {},
         body: "",
      },
      {
         url: "/home",
         method: "GET",
         headers: {},
         body: "",
      },
      {
         url: "/home",
         method: "DELETE",
         headers: {},
         body: "",
      },
      {
         url: "/home",
         method: "POST",
         headers: {},
         body: "",
      },
      {
         url: "/arb/path/-9",
         method: "GET",
         headers: {},
         body: "",
      },
   ] as Request[],

   handlersRequests: [
      {
         url: "/",
         method: "GET",
         body: "",
         headers: {},
      },
      {
         url: "/data",
         method: "POST",
         body: "",
         headers: {},
      },
   ] as Request[],
};
