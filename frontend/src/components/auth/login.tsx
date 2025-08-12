import { useState } from 'react'
import { Button } from '../ui/button'
import { Input } from '../ui/input'
import { Label } from '../ui/label'

const Login = () => {

    type UserDetails = {
        email: string;
        password: string;
    }

    const [userDet, setUserDet] = useState<UserDetails>({
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
            const response = await fetch("http://localhost:8000/api/auth/login", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(userDet),
            });

            if (response.ok) {
                console.log("Login Successful");
                let data = await response.json();
                console.log(data);

                let token = data.token;
                sessionStorage.setItem("token", token);
            }
        } catch (error: any) {
            console.log(error);
        }
    }
    return (
        <div className='flex flex-col gap-7'>
            <div className=''>
                <p className='text-black text-center text-2xl font-semibold'>Welcome Back to RustChat!</p>
                <p className='text-gray-400 text-center text-sm'>Please login to continue</p>
            </div>

            <form onSubmit={handleSubmit} action="" className='flex flex-col items-center justify-center gap-4'>

                <div className='w-[90%] flex flex-col gap-2'>
                    <Label className='text-black'>Email</Label>
                    <Input
                        name='email'
                        className="text-black"
                        value={userDet.email}
                        onChange={handleChange}
                        placeholder="johndoe@gmail.com"
                    />
                </div>
                <div className='w-[90%] flex flex-col gap-2'>
                    <Label className='text-black'>Password</Label>
                    <Input
                        name='password'
                        className="text-black"
                        placeholder="Password"
                        value={userDet.password}
                        onChange={handleChange}
                    />
                </div>

                <Button className='mt-3' type='submit'>Login</Button>
            </form>
            <div className='flex flex-col items-center justify-center'>
                <hr className='border-gray-500 w-[95%] ' />
            </div>
            <p className='text-gray-500 text-center'>Don't have an account? Need to Sign Up!</p>
        </div>
    )
}

export default Login