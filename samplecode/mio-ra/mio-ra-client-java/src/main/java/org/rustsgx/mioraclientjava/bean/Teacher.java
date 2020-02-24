package org.rustsgx.mioraclientjava.bean;

public class Teacher {
    private int id;
    private String street;
    private String city;
    private int age;
    private String sendstatus;
    private int clientid;
    private String datatype;
    private String ops;
    private int indexid;

    public int getId() {
        return id;
    }

    public void setId(int id) {
        this.id = id;
    }

    public String getStreet() {
        return street;
    }

    public void setStreet(String street) {
        this.street = street;
    }

    public String getCity() {
        return city;
    }


    public void setCity(String city) {
        this.city = city;
    }

    public int getAge() {
        return age;
    }

    public void setAge(int age) {
        this.age = age;
    }


    public String getOps() {
        return ops;
    }

    public void setOps(String ops) {
        this.ops = ops;
    }

    public String getSendstatus() {
        return sendstatus;
    }

    public void setSendstatus(String sendstatus) {
        this.sendstatus = sendstatus;
    }

    public int getClientid() {
        return clientid;
    }

    public void setClientid(int clientid) {
        this.clientid = clientid;
    }

    public String getDatatype() {
        return datatype;
    }

    public void setDatatype(String datatype) {
        this.datatype = datatype;
    }

    public int getIndexid() {
        return indexid;
    }

    public void setIndexid(int indexid) {
        this.indexid = indexid;
    }

    public void constructTeacher(int i, String sendStatus, int clientId){
        this.setId(i+clientId);
        this.setAge(i);
        this.setCity("City"+Integer.toString(i));
        this.setStreet("Street"+Integer.toString(i));
        this.setSendstatus(sendStatus);
        this.setClientid(clientId);
        this.setDatatype("energy_teacher");
        this.setOps("insert");
        this.setIndexid(i);
    }

}
