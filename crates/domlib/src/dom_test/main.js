let div = document.createElement("div");
let ul = document.createElement("ul");

div.append(ul);

div.addEventListener("click", (event) => {
  console.log("hello, world");
});

ul.addEventListener("click", () => {
    event?.target
  console.log("Apple Pie!!!!");
});

ul.onclick = () => {
  console.log("Je suis");
};

ul.click();
