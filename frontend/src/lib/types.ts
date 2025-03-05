import { Component, ReactNode } from "react";

export type Request = {
   url: string;
   method: "GET" | "POST" | "PUT" | "DELETE";
   headers: Record<string, string>;
   body: string;
   baseURL?: string;
};

export type NavLink = {
   name: string;
   to: string;
   page: () => ReactNode;
};
