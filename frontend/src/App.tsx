import {useState} from 'react'
import toast, {Toaster} from "react-hot-toast";
import {createUrl, CreateUrlData, UrlData} from "./api.ts";
import CreateUrl from "./Components/CreateUrl.tsx";
import ViewUrl from "./Components/ViewUrl.tsx";

function App() {
    const [urlData, setUrlData] = useState<UrlData | null>(null);

    const handleSendData = async (data: CreateUrlData) => {
        const response = await createUrl({
            url: data.url,
            code: data.code || undefined
        });

        if (response.status === 200) {
            toast.success("URL shorten successfully");
            setUrlData(response.data);
        } else if (response.status === 409) {
            toast.error("A URL with this code already exists");
        } else {
            toast.error("Something went wrong");
        }
    };
    return (
        <>
            <Toaster/>
            {urlData ? <ViewUrl url={urlData} /> : <CreateUrl handleSendData={handleSendData}/>}
        </>
    )
}

export default App
