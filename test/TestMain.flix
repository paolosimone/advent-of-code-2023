def add(x: Int32, y: Int32): Int32 = x + y

@Test
def testAdd01(): Bool = 0 == add(0, 0)

@Test
def testAdd02(): Bool = Assert.eq(1, add(0, 1))

@Test
def testAdd03(): Bool = Assert.eq(2, add(1, 1))

@Test
def testAdd04(): Bool = Assert.eq(4, add(1, 2))

@Test @Skip
def testAdd05(): Bool = Assert.eq(8, add(2, 3))
