// See https://aka.ms/new-console-template for more information
using System;
using System.Collections;

interface IDirFile {

    string name { get; }
    Int32 GetSize();

    List<IDirFile> children { get;}

    IDirFile parent { get; set; }
}
class MyFile : IDirFile {
    public string name { get; set; }
    public Int32 size { get; set; }

    public MyFile(String name, Int32 size) { this.name = name; this.size = size; this.children = new List<IDirFile>(); }

    public Int32 GetSize() { return size; }

    public List<IDirFile> children { get; }
    public IDirFile parent { get; set; }
}
class DirNode : IDirFile {
    public List<IDirFile> children { get; }
    public string name { get; }

    public IDirFile parent { get; set; }

    public DirNode(String name, IDirFile parent) { this.name = name; this.children = new List<IDirFile>(); this.parent = parent; }

    public Int32 GetSize()
    {
        return children.Sum(x => x.GetSize());
    }
}


class Programa {
   static void Main () {
        Console.WriteLine("Hello, World! 2");

        string[] lines = File.ReadAllLines("input.txt");
        new DirExplorerDecoder(lines);
    }
}

class DirExplorerDecoder 
{
    private String[] consoleOutputs;

    public DirExplorerDecoder(String[] consoleOutputs) 
    {
        this.consoleOutputs = consoleOutputs;
        var root = this.decodeTree();

        List<IDirFile> nodeList = new();
        this.addFilteredNodesFromTreeToList(checkGteSize, root, nodeList);

        Console.WriteLine(70000000 - root.GetSize() - 30000000);

        Console.WriteLine(nodeList.Min(x => x.GetSize()));
    }

        enum Commands 
        {
            ChangeDir,
            List,
            Dir
        }
    private IDirFile decodeTree()
    {

        bool isReadingList = false;
        IDirFile root = new DirNode("/", null);
        IDirFile curr = root;
 
        foreach (String output in this.consoleOutputs) 
        {
            String[] words = output.Split(" ");

            if (words.Length == 0)
            {
                continue;
            }

            if (words[0].Equals("$"))
            {
                isReadingList = false;
                if (words[1].Equals("cd"))
                {
                    if (words[2].Equals(".."))
                    {
                        curr = curr.parent;
                        continue;
                    }

                    if(words[2].Equals("/"))
                    {
                        
                        curr = root;
                        continue;
                    }

                    curr = curr.children.Find(x => x.name == words[2]);
                    continue;
                }

                if (words[1].Equals("ls"))
                {
                    isReadingList = true;
                    continue;
                }
            }

            if(words[0].Equals("dir"))
            {
                curr.children.Add(new DirNode(words[1], curr));
                continue;
            }

            curr.children.Add(new MyFile(words[1], Int32.Parse(words[0])));
        }

        return root;
    }

    private  bool checkGteSize( IDirFile node ) {
        return node.GetSize() >= 4125990 && node is DirNode;
    }

    private List<IDirFile> addFilteredNodesFromTreeToList(Func<IDirFile, bool> filterMethod, IDirFile node, List<IDirFile> nodeList) {
        if(filterMethod(node)) {
            nodeList.Add(node);
        }

        foreach(var childNode in node.children) {
            addFilteredNodesFromTreeToList(filterMethod, childNode, nodeList);
        }

        return nodeList;
    }
}