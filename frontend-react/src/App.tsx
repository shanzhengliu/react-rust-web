import './App.css'
import useSWR from "swr";
import axios from "axios";
import { mockInit } from './mock/lib';
const App = () =>{
  const fetcher = (url: string) => axios.get(url).then(res => res.data)
  if(import.meta.env.MODE === 'mock'){
    mockInit()
  }
  const { data,error,isLoading } = useSWR(import.meta.env.VITE_BACKEND_URL, fetcher)
  if (error) return <div>failed to load</div>
  if (isLoading) return <div>loading...</div>
  return (
    <>
      <div>
          <h1 className="text-6xl">
            {data ? data.data : "loading..."}
          </h1>
      </div>
    </>
  )
}

export default App
