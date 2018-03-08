import _ from 'lodash'
import "./style.css"
import MyImage from "./01.svg"
import Data from './data.xml'

function component() {
    var element = document.createElement('div');
  
    element.innerHTML = _.join(['Hello', 'webpack'], ' ^.^ ');
    element.classList.add("hello")
    //var myImage = new Image()
    //myImage.src = MyImage
    //element.appendChild(myImage)
    //
    console.log(Data)

    return element;
  }
  
  document.body.appendChild(component());
