import RequestResponse from "../components/RequestResponse";
import { requests } from "../lib/constants";

export default function RouterLesson() {
   return (
      <div>
         <div className="text-4xl font-bold mb-2">Router Lesson</div>
         <div className="mb-1">Test routes</div>

         <ul className="list-none">
            {requests.routerRequests.map((request, index) => (
               <li key={index}>
                  <RequestResponse request={request} />
               </li>
            ))}
         </ul>
      </div>
   );
}
