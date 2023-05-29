let div = document.createElement("div");
let ul = document.createElement("ul");
let div2 = document.createElement("div");

div.append(ul);
ul.before(div2);

div.addEventListener("click", (event) => {
   console.log("hello, world");
});

ul.onclick = () => {
   console.log("Je suis");
};

const ul_clone = ul.cloneNode();

console.log(ul_clone.previousSibling, ul.previousSibling);
