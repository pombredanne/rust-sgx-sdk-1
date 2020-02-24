package org.rustsgx.mioraclientjava.bean;

import java.util.ArrayList;
import java.util.List;

public class ComputeResult {
    private List<String> streets = new ArrayList<String>();
    private List<String> citys = new ArrayList<String>();;
    private int age;

    public List<String> getStreets() {
        return streets;
    }

    public void setStreets(List<String> streets) {
        this.streets = streets;
    }

    public List<String> getCitys() {
        return citys;
    }

    public void setCitys(List<String> citys) {
        this.citys = citys;
    }

    public int getAge() {
        return age;
    }

    public void setAge(int age) {
        this.age = age;
    }
}
