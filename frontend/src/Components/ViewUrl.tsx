import toast from "react-hot-toast";
import {UrlData} from "../api.ts";

interface ViewUrlProps {
    url: UrlData;
}

const ViewUrl = ({url}: ViewUrlProps) => {
    const handleCopy = () => {
        navigator.clipboard.writeText(`${window.location.origin}/${url.code}`).then(() => {
            toast.success("Copied to clipboard");
        });
    };

    return (
        <div className="flex flex-col items-center bg-gray-800 text-white h-screen gap-5">
            <h1 className="text-4xl font-bold mt-10">Shortened URL</h1>
            <div className="flex flex-col gap-5 items-center w-full h-full">
                <p className="text-lg">URL: {url.url}</p>
                <input readOnly={true} className="p-2 rounded-md bg-transparent border border-white"
                          value={`${window.location.origin}/${url.code}`}/>
                <button className="bg-green-700 p-2 rounded-md" onClick={handleCopy}>Copy</button>
            </div>
        </div>
    );
};

export default ViewUrl;