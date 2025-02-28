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
      console.log(2);
      axios
         .get("http://localhost:3000/")
         .then((response) => {
            setMessage(response.data);
            setIsLoading(false);
            console.log(response);
         })
         .catch((error) => {
            setError(JSON.stringify(error.response?.data || error));
            setIsLoading(false);
            console.log(error);
         });
   }, []);

   return (
      <div>
         Home
         {isLoading && <div>Loading...</div>}
         {message && <div className="text-blue-400">{message}</div>}
         {error && <div className="text-red-400">{error}</div>}
      </div>
   );
}
