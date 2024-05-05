import './App.css'
import useSWR from "swr";
import axios from "axios";
import { mockInit } from './mock/lib';
import { TableExample} from './TableExample'
const App = () =>{
  const fetcher = (url: string) => axios.get(url).then(res => res.data)
  if(import.meta.env.MODE === 'mock'){
    mockInit()
  }
  const { data,error,isLoading } = useSWR(import.meta.env.VITE_BACKEND_URL, fetcher)
  return (
    <>
      <div>

          <h1 className="text-6xl">
              {error||isLoading?"swr example":data}
          </h1>
          <TableExample />
      </div>
    </>
  )
}

export default App
