   M S B A M L   `   `   `  ����G  CScreenslimer, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null+clr-namespace:ScreenslimerScreenslimer  5   	       Screenslimer.ConfigWindow   5   	   W SPresentationCore, Version=9.0.0.0, Culture=neutral, PublicKeyToken=31bf3856ad364e35\ XPresentationFramework, Version=9.0.0.0, Culture=neutral, PublicKeyToken=31bf3856ad364e35e aSystem.Windows.Controls.Ribbon, Version=9.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089R NWindowsBase, Version=9.0.0.0, Culture=neutral, PublicKeyToken=31bf3856ad364e35F 9http://schemas.microsoft.com/winfx/2006/xaml/presentation     5   	   R NSystem.Xaml, Version=9.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089:x,http://schemas.microsoft.com/winfx/2006/xaml     5   	   @d2http://schemas.microsoft.com/expression/blend/2008     5   	   Jmc;http://schemas.openxmlformats.org/markup-compatibility/2006     5   	   ,localclr-namespace:Screenslimer     5   	     � Title$   Screenslimer Configuration��5   	   $	��450��6,   $	��800��69   .��5	      � .+�5
   
   �� $��10q�6    �� Orientation$ Vertical=�6!   .+�5      �� $	��0,2q�6   $ 
Horizontal=�6&   $
��Left=�6?   $	��Top=�6Z   .+�5      �� $��Slideshow Interval��6   $	��120��66   $
��Left=�6B   $��Center=�6]   �� 5      -   5         ��Name$ SlideshowIntervalTextBox��5      $��DefaultValue��6=   $
��Left=�6Q   $
��20,0q�6l   $	��Top=�6z   $	��120��6�   5      �� 5      $	��0,2q�6   $ 
Horizontal=�6&   $
��Left=�6?   $	��Top=�6Z   .+�5      �� $��Background Mode��6   $	��120��63   $
��Left=�6?   $��Center=�6Z   �� 5      -   5         ��Name$ BackgroundModeComboBox��5      $	��120��6;   $
��20,0q�6G   .@�5      �� $
��Auto��6#   �� 5      $��White��6#   �� 5      $��Black��6#   5      5      �� 5      $	��0,2q�6   $ 
Horizontal=�6&   $
��Left=�6?   $	��Top=�6Z   .+�5      �� $��Audio Buffer Size (KB)��6   $	��120��6:   $
��Left=�6F   $��Center=�6a   �� 5      -   5        $ BufferSizeTextBox��5      $��DefaultValue��66   $
��Left=�6J   $
��20,0q�6e   $	��Top=�6s   $	��120��6�   5      �� 5      -   5        $��Save Settings��5      $	��120��6-   $��5,20,0,0q�69   $
��Left=�6^   5       5!      5"      