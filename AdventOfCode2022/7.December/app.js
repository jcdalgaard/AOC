const fs = require("fs");

class Folder {
  constructor(parent, name) {
    this.parent = parent;
    this.name = name;
    this.content = {
      folders: [],
      files: [],
    };
    this.size = 0;
  }

  addFile(file) {
    this.content.files.push(file);
  }
  addFolder(folder) {
    this.content.folders.push(folder);
  }
  getName() {
    return this.name;
  }
  getFiles() {
    return this.content.files;
  }
  getFolders() {
    return this.content.folders;
  }
  getFolder(folderName) {
    for (var i = 0; i < this.content.folders.length; i++) {
      if (this.content.folders[i].name === folderName) {
        return this.content.folders[i];
      }
    }
  }
  getFile(fileName) {
    for (var i = 0; i < this.content.files.length; i++) {
      if (this.content.files[i].name === fileName) {
        return this.content.files[i];
      }
    }
  }
  getSize() {
    let filesSize = 0;
    this.content.files.forEach((element) => {
      filesSize += element.getSize();
    });
    let foldersize = 0;
    this.content.folders.forEach((element) => {
      foldersize += element.getSize();
    });
    this.content.size = filesSize + foldersize;
    return this.content.size;
  }
  emptyFiles() {
    this.content.files = [];
  }
  getDaddy() {
    return this.parent;
  }
  getFolderSize() {
    return this.content.size;
  }
}

class File {
  constructor(name, size) {
    this.name = name;
    this.size = Number(size);
  }
  getSize() {
    return this.size;
  }
  getName() {
    return this.name;
  }
  setSize(size) {
    this.size = size;
  }
  setName(name) {
    this.name = name;
  }
}

const setActiveFolderToParent = () => {
  if (currentFolder.getName() != "/") {
    return currentFolder.getDaddy();
  } else {
    return currentFolder;
  }
};

const readData = (data) => {
  for (let i = 0; i < data.length; i++) {
    data[i] = data[i].replace("\r", "");
    if (data[i].charAt(0) === "$") {
      handleCommand(data[i]);
    } else {
      let item = data[i].split(" ");
      if (item[0] === "dir") {
        let newFolder = new Folder(currentFolder, item[1]);
        let exists = false;
        currentFolder.getFolders().forEach((fold) => {
          if (fold.getName() === newFolder.getName()) {
            exists = true;
          }
        });

        if (!exists) {
          currentFolder.addFolder(newFolder);
          Folders.push(newFolder);
        }
      } else {
        let exists = false;
        currentFolder.getFiles().forEach((files) => {
          if (files.getName() === item[0]) {
            exists = true;
          }
        });

        if (!exists) {
          currentFolder.addFile(new File(item[1], item[0]));
        }
      }
    }
  }
};
const handleCommand = (command) => {
  if (command.split(" ")[1] === "ls") {
    lsCommand();
  } else {
    handleCd(command);
  }
};

const lsCommand = () => {
  // Nothing
};
const handleCd = (command) => {
  let com = command.split(" ");
  if (com[2] != "/") {
    currentFolder =
      com[2] === ".."
        ? setActiveFolderToParent()
        : currentFolder.getFolder(com[2]);
  }
};

try {
  const data = fs.readFileSync("t.txt", "utf8").split("\n");
  var Folders = [];
  var currentFolder = new Folder(undefined, "/");
  let asd = "helleo";
  asd = asd.replace("eo", "");

  Folders.push(currentFolder);
  readData(data);

  currentFolder = Folders[0];
  console.log(currentFolder.getSize());
  console.log(currentFolder);
  let smallfolders = [];
  for (let i = 1; i < Folders.length; i++) {
    if (Folders[i].getFolderSize() <= 100000) {
    }
    smallfolders.push(Folders[i].getFolderSize());
  }

  console.log(smallfolders);
  let sum = 0;
  sum = currentFolder.getFolderSize();
  let currentUse = 70000000 - sum;
  let bestFolderToDelete;
  let needed = 30000000;
  let val = 100000000;
  for (let i = 1; i < Folders.length; i++) {
    let temp = -1;
    temp = currentUse + Folders[i].getFolderSize();
    if (temp >= needed) {
      if (temp < val) {
        val = temp;
        bestFolderToDelete = Folders[i];
      }
    }
  }
  console.log(bestFolderToDelete.getFolderSize());
} catch (err) {
  console.log(err);
}
