import {FormEvent} from "react";
import toast from "react-hot-toast";
import {CreateUrlData} from "../api.ts";

interface CreateUrlProps {
    handleSendData: (data: CreateUrlData) => void;
}

const CreateUrl = ({handleSendData}: CreateUrlProps) => {
    const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const data = new FormData(event.currentTarget);
        const code = data.get("code");
        const url = data.get("url");
        if (!url) {
            return toast.error("Please fill in all fields");
        }
        handleSendData({url: url.toString(), code: code?.toString() || undefined});
    };

    return (
        <div className="flex flex-col items-center bg-gray-800 text-white h-screen gap-5">
            <h1 className="text-4xl font-bold mt-10">Shorten URL</h1>
            <form onSubmit={handleSubmit} className="flex flex-col gap-5 items-center w-full h-full">
                <input className="p-2 rounded-md bg-transparent border border-white" placeholder="URL"
                       name="url"/>
                <input className="p-2 rounded-md bg-transparent border border-white w-full" placeholder="Code" name="code"/>
                <button className="bg-green-700 p-2 rounded-md" type="submit">Shorten</button>
            </form>

        </div>
    )
};

export default CreateUrl;