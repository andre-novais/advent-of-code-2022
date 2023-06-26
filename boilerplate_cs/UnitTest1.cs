namespace boilerplate_cs;
using System;
using System.Collections;
using Solution;

public class Tests
{
    [SetUp]
    public void Setup()
    {
    }

    [Test]
    public void partA()
    {
        var inputLines = File.ReadAllLines(Path.Combine(TestContext.CurrentContext.TestDirectory, "../../../input.txt"));

        var solution = new PartA(inputLines).Solve();
        Assert.AreEqual(solution, "100");
    }

    public void partB()
    {
        var inputLines = File.ReadAllLines("../../../input.txt");

        var solution = new PartB(inputLines).Solve();
        Assert.AreEqual(solution, "100");
    }
}