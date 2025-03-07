import ExtractorsLesson from "../pages/ExtractorsLesson";
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
   {
      name: "Extractors",
      to: "/extractors",
      page: ExtractorsLesson,
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

   extractorsRequests: [
      { url: "/typed-header", method: "GET", body: "", headers: {} },
      { url: "/", method: "GET", body: "", headers: {} },
      {
         url: "/signup",
         method: "POST",
         body: JSON.stringify({ password: "password4", email: "someemail@email.com" }),
         headers: {},
      },
      {
         url: "/signup",
         method: "POST",
         body: JSON.stringify({ username: "password4", email: "someemail@email.com" }),
         headers: {},
      },
      {
         url: "/signup",
         method: "POST",
         body: JSON.stringify({ username: "johnny doef", password: "password4", email: "someemail@email.com" }),
         headers: {},
      },
      {
         url: "/signup",
         method: "POST",
         body: JSON.stringify({ username: "janine doefele", password: "passwo", email: "someemail@email.com" }),
         headers: {},
      },
      { url: "/path/22", method: "POST", body: "", headers: {} },
      { url: "/query?q=fish&match=no-match&locale=en-GB", method: "POST", body: "", headers: {} },
      { url: "/combined/455-43-53-XC3?property_type=condo&location=Gweru&property_status=occupied", method: "GET", body: "", headers: {} },
      { url: "/combined/455-43-53-XC3?page=1&property_status=occupied", method: "GET", body: "", headers: {} },
      { url: "/headers", method: "POST", body: "", headers: {} },
      { url: "/string", method: "POST", body: "hello stringified", headers: {} },
      { url: "/matched-path", method: "POST", body: "", headers: {} },
      { url: "/original-uri", method: "POST", body: "", headers: {} },
   ] as Request[],
};
