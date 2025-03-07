import { prettyPrintJson } from "pretty-print-json";
import React from "react";
import axios, { AxiosError, AxiosResponse } from "axios";
import { Request } from "../lib/types";
import { axiosClient } from "../lib/constants";
import { ChevronDown, ChevronUp } from "lucide-react";

type propTypes = {
   request: Request;
};

export default function RequestResponse({ request }: propTypes) {
   const [html, setHtml] = React.useState<string | null>(null);
   const [loading, setLoading] = React.useState<boolean>(true);
   const [isError, setIsError] = React.useState<boolean>(false);
   const [res, setRes] = React.useState<AxiosResponse | AxiosError | null>(null);
   const [expanded, setExpanded] = React.useState<boolean>(false);

   React.useEffect(() => {
      setHtml(null);
      setRes(null);
      setIsError(false);
      setLoading(true);
      axiosClient({
         url: request.url,
         method: request.method,
         headers: request.headers,
         data: request.body,
         ...(request.baseURL && { baseURL: request.baseURL }),
      })
         .then((res) => {
            if (res.headers["content-type"]?.includes("html")) {
               setHtml(res.data);
            } else {
               setHtml(prettyPrintJson.toHtml(res.data));
            }
            setRes(res);
            setLoading(false);
         })
         .catch((error) => {
            const err = error;
            if (err.config) {
               delete err.config;
            }
            if (err.stack) {
               delete err.stack;
            }
            if (axios.isAxiosError(err) && err.response?.headers["content-type"]?.includes("html")) {
               setHtml(err.response.data);
            } else {
               setHtml(prettyPrintJson.toHtml(err.response?.data || err));
            }

            setRes(err);
            setLoading(false);
            setIsError(true);
         });
   }, [request.baseURL, request.body, request.headers, request.method, request.url]);

   return (
      <div className="shadow-lg bg-[#0b0b16]  rounded-lg my-3  p-4">
         <div className="flex gap-3 flex-wrap align-items-center mb-3">
            <div className="text-blue-900  bg-blue-100  p-1 px-2 font-semibold rounded">
               <span className="font-semibold">URL:</span> {request.url}
            </div>
            <div className="text-green-900  bg-green-100  p-1 px-2 font-semibold rounded">
               <span className="font-semibold">Method:</span> {request.method}
            </div>
            <div className="text-purple-900  bg-purple-100  p-1 px-2 font-semibold rounded">
               <span className="font-semibold">Headers:</span> {JSON.stringify(request.headers, null, 2)}
            </div>
            <div className="text-red-900  bg-red-100  p-1 px-2 font-semibold rounded">
               <span className="font-semibold">Body:</span> {request.body}
            </div>
         </div>

         {loading ? (
            <div className="flex justify-center items-center min-w-[500px] w-fit p-4">
               <div className="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-blue-500"></div>
            </div>
         ) : (
            <div className="flex items-start gap-3">
               <div
                  className={`grow p-4 border shadow-lg  rounded-xl 
                  
                  ${isError ? " border-red-200" : " border-green-500"}`}
               >
                  {html && (
                     <>
                        <pre className={`whitespace-pre-wrap break-words overflow-auto ${expanded ? "max-h-fit" : "max-h-[15rem]"}`} dangerouslySetInnerHTML={{ __html: html }} />
                     </>
                  )}
               </div>

               {res && (
                  <div className="ps-2 flex  gap-2  text-nowrap">
                     <button onClick={() => setExpanded((prev) => !prev)} className="text-xs p-0.5 rounded bg-blue-800 text-blue-200">
                        {expanded ? <ChevronUp size={15} /> : <ChevronDown size={15} />}
                     </button>
                     <div className={`text-lg p-1 rounded ${isError ? "bg-red-800 text-red-200" : "bg-green-900 text-green-200"}`}>
                        <span className="font-semibold">Status:</span> {res.status} <br />
                     </div>
                     <div className={`text-lg p-1 rounded ${isError ? "bg-purple-900 text-red-200" : "bg-green-700 text-green-200"}`}>
                        <span className="font-semibold"> {axios.isAxiosError(res) ? res.response?.statusText : (res as AxiosResponse).statusText}</span>
                     </div>
                  </div>
               )}
            </div>
         )}
      </div>
   );
}
