using System;

namespace DistanceFieldComputer
{
    internal class Point
    {
        public float distance;
        public int x;
        public int y;

        public Point(int x, int y)
        {
            this.x = x;
            this.y = y;
        }

        public void ComputeDistanceToOrigin()
        {
            distance = (float) Math.Sqrt(Math.Pow(Math.Abs(x), 2) + Math.Pow(Math.Abs(y), 2));
        }
    }
}