const bbb = function(){ return 'string'}

async function funAsy() {
   console.log("no await before")
   const a = await 1
   const b = await new Promise((resolve, reject)=>{
        setTimeout(function(){
           resolve('time')
        }, 3000)
   })
   const c = await bbb()
   console.log(a)
   console.log(b)
   console.log(c)
   console.log("no await after")
}

funAsy() 
console.log("after async")
