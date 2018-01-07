#pragma once

enum class EnumClass: char {
    EC_First,
    EC_Second
};

enum EnumSimple {
    ES_First,
    ES_Second
};

class Test {
public:
	int DoTest() const {}
	virtual void PureVirtual() = 0;
};

class SubTest : public Test {
public:
    virtual void PureVirtual() {
    }
};