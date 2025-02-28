import axios from "axios";
import { useEffect, useState } from "react";

export default function Home() {
   const [message, setMessage] = useState<string | null>(null);
   const [error, setError] = useState<string | null>(null);
   const [isLoading, setIsLoading] = useState(true);

   useEffect(() => {
      setError(null);
      setMessage(null);
      setIsLoading(true);
      axios
         .get("http://localhost:3000/")
         .then((response) => {
            setMessage(response.data.message);
            setIsLoading(false);
         })
         .catch((error) => {
            setError(JSON.stringify(error.response?.data || error));
            setIsLoading(false);
         });
   }, []);

   return (
      <div>
         Home
         {isLoading && <div>Loading...</div>}
         <div className="text-[blue]">{message}</div>
         <div className="text-[red]">{error}</div>
      </div>
   );
}
