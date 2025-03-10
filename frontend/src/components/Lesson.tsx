import RequestResponse from "../components/RequestResponse";
import { NavLink } from "../lib/types";

export default function LessonPage({ lesson }: { lesson: NavLink }) {
   return (
      <div>
         <div className="text-4xl font-bold mb-2">{lesson.name} Lesson</div>
         <div className="mb-1">Test routes</div>

         <ul className="list-none">
            {lesson.requests.map((request, index) => (
               <li key={index}>
                  <RequestResponse request={request} />
               </li>
            ))}
         </ul>
      </div>
   );
}
