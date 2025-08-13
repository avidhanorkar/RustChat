
type navprops = {
    username: string;
}

const Navbar = ({username}: navprops) => {
    return (
        <div className='flex flex-row items-center justify-around gap-2 bg-black h-[10vh] w-full text-white border-b border-gray-500'>
            <p className='text-white text-2xl font-semibold'>RustChat</p>
            <div className='flex flex-row items-center gap-2'>
                <p className=''>{username}</p>
                <div className='cursor-pointer'>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                        <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                    </svg>
                </div>
            </div>
        </div>
    )
}

export default Navbar