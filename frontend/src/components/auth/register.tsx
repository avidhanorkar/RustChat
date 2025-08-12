import { useState } from 'react'
import { Input } from '../ui/input'
import { Label } from '../ui/label'
import { Button } from '../ui/button'

const Register = () => {
    type UserDetails = {
        name: string;
        email: string;
        password: string;
    }

    const [userDet, setUserDet] = useState<UserDetails>({
        name: "",
        email: "",
        password: "",
    });

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setUserDet((prev) => ({
            ...prev,
            [name]: value,
        }));
    };

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        try {
            const response = await fetch("http://localhost:8000/api/auth/register", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(userDet),
            });

            if (response.ok) {
                console.log("Register Successful");
                let data = await response.json();
                console.log(data);
                sessionStorage.setItem("token", data.token);
            }
        } catch (error: any) {
            console.log(error);
        }
    }

    return (
        <div className='flex flex-col gap-7 '>
            <div className=''>
                <p className='text-black text-center text-2xl font-semibold'>Welcome to RustChat!</p>
                <p className='text-gray-400 text-center text-sm'>Please register to continue</p>
            </div>

            <form onSubmit={handleSubmit} action="" className='flex flex-col items-center justify-center gap-4'>

                <div className='w-[90%] flex flex-col gap-2'>
                    <Label className='text-black'>Name</Label>
                    <Input className="text-black" placeholder="John Doe" name='name' value={userDet.name} onChange={handleChange} />
                </div>
                <div className='w-[90%] flex flex-col gap-2'>
                    <Label className='text-black'>Email</Label>
                    <Input className="text-black" placeholder="johndoe@gmail.com" name='email' value={userDet.email} onChange={handleChange} />
                </div>
                <div className='w-[90%] flex flex-col gap-2'>
                    <Label className='text-black'>Password</Label>
                    <Input className="text-black" placeholder="Password" name='password' value={userDet.password} onChange={handleChange} />
                </div>

                <Button type='submit' className='hover::border-0'>Register</Button>
            </form>
            <div className='flex flex-col items-center justify-center'>
                <hr className='border-gray-500 w-[95%] ' />
            </div>
            <p className='text-gray-500 text-center'>Already have an account? Need to Login!</p>
        </div>
    )
}

export default Register