package org.rustsgx.mioraclientjava.raverify;

import java.io.*;
import java.util.ArrayList;
import java.util.List;

public class CommonUtils {
    public static List<Byte> string2BytesList(String[] strings) {
        ArrayList<Byte> arrayList = new ArrayList<Byte>();
        for (int i = 0; i < strings.length; i++) {
            int intVal = Integer.decode(strings[i]);
            arrayList.add(Byte.valueOf((byte) intVal));
        }
        return arrayList;
    }

    public static int getIndexOf(List<Byte> b, List<Byte> bb) {
        if (b == null || bb == null || b.size() == 0 || bb.size() == 0 || b.size() < bb.size())
            return -1;

        int i, j;
        for (i = 0; i < b.size() - bb.size() + 1; i++) {
            if (b.get(i) == bb.get(0)) {
                for (j = 1; j < bb.size(); j++) {
                    if (b.get(i + j) != bb.get(j))
                        break;
                }
                if (j == bb.size())
                    return i;
            }
        }
        return -1;
    }

    public static byte hexToByte(String inHex) {
        return (byte) Integer.parseInt(inHex, 16);
    }

    public static String byteToHex(byte b) {
        String hex = Integer.toHexString(b & 0xFF);
        if (hex.length() == 1) {
            hex = "0" + hex;
        }
        return hex;
    }

    public static byte[] list2array(List<Byte> list) {
        byte[] bytes = new byte[list.size()];
        for (int i = 0; i < list.size(); i++) {
            bytes[i] = list.get(i);
        }
        return bytes;
    }

    public static void printCert(byte[] rawByte) {
        System.out.print("---received-server cert: [Certificate(b\"");
        for (int i = 0; i < rawByte.length; i++) {
            char c = (char) (Byte.toUnsignedInt(rawByte[i]));
            if (c == '\n') {
                System.out.print("\\n");
            } else if (c == '\r') {
                System.out.print("\\r");
            } else if (c == '\t') {
                System.out.print("\\t");
            } else if (c == '\\' || c == '"') {
                System.out.printf("\\%c", c);
            } else if (Byte.toUnsignedInt(rawByte[i]) >= 32 && Byte.toUnsignedInt(rawByte[i]) < 127) {
                System.out.printf("%c", c);
            } else {
                System.out.printf("\\x%02x", rawByte[i]);
            }
        }
        System.out.println("\")]");
    }

    public static void writeInFileByfb(String content,String fileName) {
        File f=new File(fileName);
        FileWriter fw=null;
        BufferedWriter bw=null;
        try{
            if(!f.exists()){
                f.createNewFile();
            }
            fw=new FileWriter(f.getAbsoluteFile());
            bw=new BufferedWriter(fw);
            bw.write(content);
            bw.close();
        }catch(Exception e){
            e.printStackTrace();
        }
    }

    public static String bytesToHex(byte[] bytes) {
        StringBuffer sb = new StringBuffer();
        for(int i = 0; i < bytes.length; i++) {
            String hex = Integer.toHexString(bytes[i] & 0xFF);
            if(hex.length() < 2){
                sb.append(0);
            }
            sb.append(hex);
        }
        return sb.toString();
    }

    public static String readFileReturnFirstLine(String fileName) {
        File file = new File(fileName);
        BufferedReader reader = null;
        try {
            reader = new BufferedReader(new FileReader(file));
            String tempString = null;
            String firstLine = null;
            int line = 1;
            // 一次读入一行，直到读入null为文件结束
            while ((tempString = reader.readLine()) != null) {
                // 显示行号
                if(line == 1){
                    firstLine = tempString;
                }
                line++;
            }
            reader.close();
            return firstLine;
        } catch (IOException e) {
            e.printStackTrace();
        } finally {
            if (reader != null) {
                try {
                    reader.close();
                } catch (IOException e1) {
                }
            }
        }
        return "";
    }
}