#include <gtest/gtest.h>

class MyClass {
};

class Compositor : public ::testing::Test {
protected:
    virtual void SetUp() {
        myclass = new MyClass();
    };

    virtual void TearDown() {
        delete myclass;
    };

    MyClass* myclass;
};

TEST_F(Compositor, foo) {
    ASSERT_EQ(3, 3);
}
