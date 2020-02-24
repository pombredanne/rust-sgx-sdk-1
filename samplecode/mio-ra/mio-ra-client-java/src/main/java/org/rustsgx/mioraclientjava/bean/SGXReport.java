package org.rustsgx.mioraclientjava.bean;

public class SGXReport {
    private String report;
    private String pubkey;
    private String hmacString;

    public String getReport() {
        return report;
    }

    public void setReport(String report) {
        this.report = report;
    }

    public String getPubkey() {
        return pubkey;
    }

    public void setPubkey(String pubkey) {
        this.pubkey = pubkey;
    }

    public String getHmacString() {
        return hmacString;
    }

    public void setHmacString(String hmacString) {
        this.hmacString = hmacString;
    }
}
