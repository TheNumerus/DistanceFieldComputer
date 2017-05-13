using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading;

namespace DistanceFieldComputer
{
    class Bucket
    {
        public List<Point> points;
        public int x;
        public int y;
        public int progress;
        public Bucket(int x, int y) {
            this.x = x;
            this.y = y;
            points = new List<Point>();
        }
        public void Fill(int imgWidth, int imgHeight, int radius) {
            for (var _x = x * radius; _x < (x + 1) * radius; _x++)
            for (var _y = y * radius; _y < (y + 1) * radius; _y++) {
                if(_x >= 0 && _y >= 0 && _x < imgWidth && _y < imgHeight)
                    points.Add(new Point(_x, _y));
            }
        }
    }
}
