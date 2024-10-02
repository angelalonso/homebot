// https://gamedev.stackexchange.com/questions/37680/pattern-for-performing-game-actions

abstract class Action
{
    abstract void Update(float elapsed);
    bool Finished;
}

abstract class CompositeAction : Action
{
    void Add(Action action) { 
        Actions.Add(action);
    }
    List<Action> Actions;
}

class Parallel : CompositeAction
{
    override void Update(float elapsed) 
    {
        Actions.ForEach(a=> a.Update(elapsed));
        Actions.RemoveAll(a => a.Finished);
        Finished = Actions.Count == 0;
    }
}

class Sequence : CompositeAction
{
    override void Update(float elapsed) 
    {
        if (Actions.Count > 0) 
        {
            Actions[0].Update(elapsed);
            if (Actions[0].Finished) {
                Actions.RemoveAt(0);
            }
        }
        Finished = Actions.Count == 0;
    }
}
