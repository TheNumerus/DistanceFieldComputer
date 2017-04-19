using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DistanceFieldComputer
{
    class Point
    {
        public int x;
        public int y;
        public float distance;
        public Point(int x, int y,float distance)
        {
            this.x = x;
            this.y = y;
            this.distance = distance;
        }
        public Point(int x, int y)
        {
            this.x = x;
            this.y = y;
        }
        public void computeDistanceToOrigin()
        {
            distance = (float)Math.Sqrt(Math.Pow(Math.Abs(x),2) + Math.Pow(Math.Abs(y),2));
        }
    }
}
