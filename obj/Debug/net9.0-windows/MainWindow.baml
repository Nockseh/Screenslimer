   M S B A M L   `   `   `  ����G  CScreenslimer, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null+clr-namespace:ScreenslimerScreenslimer  5   	       Screenslimer.MainWindow   5   	   W SPresentationCore, Version=9.0.0.0, Culture=neutral, PublicKeyToken=31bf3856ad364e35\ XPresentationFramework, Version=9.0.0.0, Culture=neutral, PublicKeyToken=31bf3856ad364e35e aSystem.Windows.Controls.Ribbon, Version=9.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089R NWindowsBase, Version=9.0.0.0, Culture=neutral, PublicKeyToken=31bf3856ad364e35F 9http://schemas.microsoft.com/winfx/2006/xaml/presentation     5   	   R NSystem.Xaml, Version=9.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089:x,http://schemas.microsoft.com/winfx/2006/xaml     5   	   @d2http://schemas.microsoft.com/expression/blend/2008     5   	   Jmc;http://schemas.openxmlformats.org/markup-compatibility/2006     5   	   ,localclr-namespace:Screenslimer     5   	   -   5          � Title$  
MainWindow��5   	   $	��360��6   $	��640��6)   $
|�True��5	   	   .��5      � .+�5   
   �� -   5         ��Name$ SlideshowImage��5   ?   $+��%C:\Users\Nockseh\Pictures\critter.jpg��6   $��Uniform=�6W   $��2��6i   �� 5   
   -   5         ��Name$ BackgroundRectangle��5      
���   � K� ZIndex$ -1��6>   $��2��6P   �� 5   
   $
��Left=�6   $	��Top=�60   $��10q�6H    �� Orientation$ Vertical=�6T   .+�5      (�  (� Stretch$ Uniform=�6   $��40��6(   $
��Left=�64   .��5      �� -   5         ��Name$ 	SongLabel��5      $��🎵 Song Title��5       �� TextAlignment$
 Left=�5      $
��Left=�5      
������� J� Effect 5      3  -System.Windows.Media.Effects.DropShadowEffect  5      	   Color$	 Black��6+   
   
BlurRadius$
 1Q�69      ShadowDepth$ 1Q�6H      	Direction$	 315Q�6X   5      5      (� 5      $ Uniform=�6   $��30��6(   $
��Left=�64   .��5      �� -   5        $ 	BandLabel��5      $��👤 Artist Name��5       $
 Left=�5!      $
��Left=�5"      
������� 5$        5%      $	 Black��6+   $
 1Q�69   $ 1Q�6H   $	 315Q�6X   5'      5(      (� 5)      $ Uniform=�6   $��24��6(   $
��Left=�64   .��5*      �� -   5        $ 
AlbumLabel��5*      $��💿 Album Name��5+      $
 Left=�5,      $
��Left=�5-      
������� 5/        50      $	 Black��6+   $
 1Q�69   $ 1Q�6H   $	 315Q�6X   52      53      54      �� 57   
   $��Right=�6   $��Bottom=�61   $��10q�6L   .+�58      (� $ Uniform=�6   $��24��6(   .��59      �� -   5        $ ArtistLabel��59      $��ArtistLabel��5:      
�������$ Right=�5<      $��NoWrap=�5=       5>        5?      $	 Black��6+   $
 1Q�69   $ 1Q�6H   $	 315Q�6X   5A      5B      5C      5E      5F      