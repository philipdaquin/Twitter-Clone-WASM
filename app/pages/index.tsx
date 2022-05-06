import type { NextPage } from 'next'
import Head from 'next/head'
import Image from 'next/image'
import Sidebar from '../components/sidebar'


const Home: NextPage = () => {
  return (
    <div className="">
      <Head>
        <title>Real Time Blogging Service on Next Js and Rust</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      {/* Side bar */}
      <Sidebar />
      {/*  Feed */}

      {/* Widgets */}
      
      <main></main>
    </div>
  )
}

export default Home
