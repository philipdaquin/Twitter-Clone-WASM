import React, { useRef, useState } from 'react'

import {
    SearchCircleIcon,
    EmojiHappyIcon,
    CalendarIcon,
    LocationMarkerIcon,
    PhotographIcon
} from '@heroicons/react/outline'
import { useSession } from 'next-auth/react';


function TweetBox() {
    const [input, setInput] = useState<string>('');
    const [image, setImageURL] = useState<string>('');
    const { data: session } = useSession();

    const ImageInputRef = useRef<HTMLInputElement>(null);

    const [imageUrlEmpty, setImage] = useState<boolean>(false); 

    const addImageToTweet = (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => { 
        //  Prevent the browser from checking 
        e.preventDefault()
        //  Check if the input ref is empty, return the value else do nothing 
        if (!ImageInputRef.current?.value) return ;
        //  else, set the image url to the current value of the input ref 
        setImageURL(ImageInputRef.current.value)
        // Hide the imageinputref to blank
        ImageInputRef.current.value = '';
        //  reset the current variable to default
        setImage(false);
    }

    return (
        <div className='flex space-x-2 p-5'>
            <img 
                className="h-14 w-14 object-cover rounded-full mt-4 " 
                src={ session?.user?.image || "https://links.papareact.com/gll"} 
                alt="" 
            />
            <div className='flex flex-1 items-center pl-2'>
                <form action="" className='flex flex-1 flex-col'>
                    <input
                        value={input}
                        onChange={e => setInput(e.target.value)}
                        className='h-24 w-full text-xl outline-none placeholder:text-xl' 
                        type="text" 
                        placeholder="What's Happening?"  />
                    <div className="flex items-center">
                        <div className='flex flex-1 space-x-2 text-twitter' >
                            <PhotographIcon 
                                onClick={() => setImage(!imageUrlEmpty)}
                                className='h-5 w-5 cursor-pointer 
                                transition-transform duration-150 ease-out hover:scale-150'/>  
                            <SearchCircleIcon className='h-5 w-5'/>  
                            <EmojiHappyIcon className='h-5 w-5'/>  
                            <CalendarIcon className='h-5 w-5'/>  
                            <LocationMarkerIcon className='h-5 w-5'/>  
                        </div>
                        <button 
                            disabled={!input || !session}
                            className="bg-twitter px-5 py-2 font-bold 
                                rounded-full text-white disabled:opacity-40"
                            >Tweet
                        </button>
                    </div>
                    {
                        imageUrlEmpty &&  (
                            <form className='mt-5 flex rounded-lg bg-twitter/80 py-2 px-4'>
                                <input 
                                    ref={ImageInputRef}
                                    className="flex-1 bg-transparent p-2 
                                    text-white outline-none placeholder:text-white" type="text" placeholder="Enter Image URL"/>
                                <button 
                                    onClick={addImageToTweet}
                                    type="submit" 
                                    className="font-bold text-white">Add Image</button>                             
                            </form>
                        )
                    }

                    {image && ( 
                        <img className="mt-20 h-70 w-50 rounded-xl shadow-lg" src={image} alt="" />
                    )} 
                </form>
            </div>
        </div>
    )
}




export default TweetBox