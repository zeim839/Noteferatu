import * as React from "react"
import { Toolbar } from "./toolbar"
import { MDModeProvider } from "./context"

// MDMode is the markdown-mode editor.
function MDMode() {
  return (
    <MDModeProvider>
      <div className="flex flex-col w-full bg-[#EFF1F5] max-h-[calc(100vh-35px-30px)] h-[calc(100vh-35px-30px)]">
        <Toolbar />
        <div className="flex flex-col pt-5 w-full px-5 gap-5 leading-7 overflow-y-auto">
          <h1 className="text-3xl">Introduction</h1>
          <p>
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin in
            tellus sodales lorem tincidunt tempus non vitae urna. Cras id arcu
            eu leo efficitur pellentesque. Ut fermentum molestie elit eu
            facilisis. Fusce commodo imperdiet nisl, id euismod ligula lobortis
            et. Duis mollis ex purus. Morbi lobortis urna a maximus efficitur.
            Quisque odio mauris, hendrerit id risus vel, fermentum mattis
            nulla. Quisque vel imperdiet justo.
            <br />
            <br />
            Aenean ante eros, tincidunt in ante sollicitudin, molestie ultrices
            metus. Integer vitae tincidunt velit, eget ullamcorper sapien. Orci
            varius natoque penatibus et magnis dis parturient montes, nascetur
            ridiculus mus. Donec ipsum mi, faucibus sit amet tellus ac, auctor
            facilisis leo. Ut vestibulum molestie nulla, et luctus ipsum
            volutpat sed. In hac habitasse platea dictumst. Praesent arcu nisi,
            laoreet non fringilla non, varius vel mauris. Donec iaculis velit
            nec nisl venenatis, eget ornare nisi dapibus. Lorem ipsum dolor sit
            amet, consectetur adipiscing elit. Sed nibh velit, dapibus molestie
            imperdiet vel, commodo id sapien. Aenean in dolor vehicula, dictum
            tellus non, hendrerit nibh.
          </p>
        </div>
      </div>
    </MDModeProvider>
  )
}

export { MDMode }
