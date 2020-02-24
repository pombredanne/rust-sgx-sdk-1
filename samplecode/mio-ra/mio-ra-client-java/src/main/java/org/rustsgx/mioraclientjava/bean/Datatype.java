package org.rustsgx.mioraclientjava.bean;

public class Datatype {

    public String types;

    public String sendStatus;

    public int clientId;

    public String getTypes() {
        return types;
    }

    public void setTypes(String types) {
        this.types = types;
    }

    public String getSendStatus() {
        return sendStatus;
    }

    public void setSendStatus(String sendStatus) {
        this.sendStatus = sendStatus;
    }

    public int getClientId() {
        return clientId;
    }

    public void setClientId(int clientId) {
        this.clientId = clientId;
    }

    public void constructDatatype(String types,String sendStatus,int clientId){
        this.setTypes(types);
        this.setSendStatus(sendStatus);
        this.setClientId(clientId);
    }
}
